use std::{borrow::Cow, collections::HashMap};

use darling::{FromDeriveInput, FromVariant, ast::{Data, Fields}};
use heck::SnakeCase;
use proc_macro2::Span;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Generics, Ident, Type, Visibility};

use crate::{Relationship, relationship::RelationshipType};

#[derive(FromDeriveInput)]
#[darling(attributes(stix), supports(enum_newtype))]
pub struct Collection {
    ident: Ident,
    vis: Visibility,
    generics: Generics,
    data: Data<Variant, ()>,
}

impl Collection {
    /// Convenience function for mapping over all the resource types this declaration supports
    fn variants_as<'a, T>(&'a self, map: impl Fn(&'a Variant) -> T) -> Vec<T> {
        self.data
            .as_ref()
            .map_enum_variants(map)
            .take_enum()
            .unwrap()
    }

    fn to_rel_matrix<'a>(&'a self) -> RelMatrix<'a> {
        let objects_by_ident = self.variants_as(|v| (&v.ident, v)).into_iter().collect::<HashMap<_, _>>();
        let tuples = self.variants_as(|v| v.to_rel_tuples()).into_iter().flatten().collect();
        RelMatrix {
            objects_by_ident,
            tuples,
        }
    }
}

impl ToTokens for Collection {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let vis = &self.vis;
        let (_, ty_generics, where_clause) = self.generics.split_for_impl();
        let builder_match_arms = self.variants_as(|v| BuilderMatchArm::new(ident, v));
        let store_fields = self.variants_as(FieldDeclaration);
        let resource_iters = self.variants_as(ResourceIter);
        let rel_matrix = self.to_rel_matrix();

        tokens.append_all(quote! {
            #[derive(Default)]
            #vis struct CollectionBuilder #ty_generics #where_clause {
                #(#store_fields),*
            }

            impl CollectionBuilder {
                pub fn add_bundle(&mut self, bundle: ::stix::Bundle<#ident>) {
                    for declaration in bundle.objects {
                        match declaration {
                            #(#builder_match_arms),*
                        }
                    }
                }

                pub fn build(self) -> Collection {
                    Collection { data: CollectionData::new(self) }
                }
            }

            impl<'a> Into<::stix::RelationshipGraph<'a>> for &'a CollectionBuilder {
                fn into(self) -> ::stix::RelationshipGraph<'a> {
                    self.relationships.values().collect()
                }
            }

            ::stix::export::sync_once_self_cell!(CollectionData, CollectionBuilder, ::stix::RelationshipGraph<'_>);

            #vis struct Collection {
                data: CollectionData
            }

            impl Collection {
                #(#resource_iters)*
            }

            impl Collection {
                fn data(&self) -> &CollectionBuilder {
                    self.data.get_owner()
                }

                fn graph(&self) -> &::stix::RelationshipGraph {
                    self.data.get_or_init_dependent()
                }
            }

            impl From<::stix::Bundle<#ident>> for Collection {
                fn from(bundle: ::stix::Bundle<#ident>) -> Self {
                    let mut builder = CollectionBuilder::default();
                    builder.add_bundle(bundle);
                    builder.build()
                }
            }

            #vis struct Node<'a, D> {
                data: &'a D,
                collection: &'a Collection,
            }

            impl<'a, D> Node<'a, D> {
                fn new(data: &'a D, collection: &'a Collection) -> Self {
                    Self { data, collection }
                }

                fn link<E>(&'a self, data: &'a E) -> Node<'a, E> {
                    Node::new(data, self.collection)
                }
            }

            impl<'a, D> ::std::ops::Deref for Node<'a, D> {
                type Target = D;
            
                fn deref(&self) -> &Self::Target {
                    self.data
                }
            }

            impl<D: AsRef<::stix::CommonProperties>> AsRef<::stix::CommonProperties> for Node<'_, D> {
                fn as_ref(&self) -> &::stix::CommonProperties {
                    self.data.as_ref()
                }
            }

            #rel_matrix

        });
    }
}

struct BuilderMatchArm<'a> {
    declaration_ident: &'a Ident,
    variant: &'a Variant,
}

impl<'a> BuilderMatchArm<'a> {
    fn new(decl: &'a Ident, variant: &'a Variant) -> Self {
        Self {
            declaration_ident: decl,
            variant,
        }
    }
}

impl ToTokens for BuilderMatchArm<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let enum_ident = self.declaration_ident;
        let variant_ident = &self.variant.ident;
        let dest = self.variant.set_name();

        if self.variant.has_value() {
            tokens.append_all(quote! {
                #enum_ident::#variant_ident(v) => { self.#dest.insert(::stix::Object::id(&v).clone(), v); }
            });
        } else {
            // Known-ignored declaration types are handled to ensure that the enum match remains exhaustive
            tokens.append_all(quote! {
                #enum_ident::#variant_ident => {}
            });
        }
    }
}

#[derive(FromVariant)]
#[darling(attributes(stix))]
pub struct Variant {
    ident: Ident,
    fields: Fields<Type>,
    #[darling(default)]
    set_name: Option<Ident>,
    #[darling(default, multiple)]
    rel: Vec<Relationship>,
}

impl Variant {
    pub fn has_value(&self) -> bool {
        self.ty().is_some()
    }

    pub fn ty(&self) -> Option<&Type> {
        if self.fields.is_newtype() {
            Some(&self.fields.fields[0])
        } else {
            None
        }
    }

    pub fn set_name<'a>(&'a self) -> Cow<'a, Ident> {
        if let Some(set_name) = &self.set_name {
            Cow::Borrowed(set_name)
        } else {
            Cow::Owned(Ident::new(
                &crate::pluralize(&self.ident.to_string().to_snake_case()),
                self.ident.span(),
            ))
        }
    }

    pub fn to_rel_tuples<'a>(&'a self) -> impl Iterator<Item = RelTuple<'a>> {
        self.rel.iter().map(move |rel| (&self.ident, &rel.rel, &rel.to))
    }
}

struct FieldDeclaration<'a>(&'a Variant);

impl ToTokens for FieldDeclaration<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let var_name = self.0.set_name();
        if let Some(ty) = self.0.ty() {
            tokens.append_all(quote!(#var_name: ::stix::export::IndexMap<::stix::Id, #ty>));
        }
    }
}

struct ResourceIter<'a>(&'a Variant);

impl ToTokens for ResourceIter<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(ty) = self.0.ty() {
            let method_name = self.0.set_name();

            tokens.append_all(quote! {
                pub fn #method_name<'a>(&'a self) -> impl Iterator<Item = Node<'a, #ty>> {
                    self.data.get_owner().#method_name.values().map(move |v| Node::new(v, self))
                }
            })
        }
    }
}

struct RelMatrix<'a> {
    objects_by_ident: HashMap<&'a Ident, &'a Variant>,
    tuples: Vec<RelTuple<'a>>,
}

impl ToTokens for RelMatrix<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for (ident, variant) in &self.objects_by_ident {
            let ident_str = ident.to_string();
            
            let forward_rows = self.tuples.iter().filter(|tup| tup.0 == &ident_str).map(|i| RelMatrixItem {
                rel: &i.1,
                is_reversed: false,
                dest: self.objects_by_ident.get(i.2).copied().ok_or_else(|| variant_not_found(i.2).write_errors())
            }).collect::<Vec<_>>();

            let reverse_rows = self.tuples.iter().filter(|tup| tup.2 == &ident_str).map(|i| RelMatrixItem {
                rel: &i.1,
                is_reversed: true,
                dest: self.objects_by_ident.get(i.0).copied().ok_or_else(|| variant_not_found(i.0).write_errors())
            }).collect::<Vec<_>>();

            if forward_rows.is_empty() && reverse_rows.is_empty() {
                continue;
            }

            let ty = variant.ty();

            tokens.append_all(quote! {
                impl<'a> Node<'a, #ty> {
                    #(#forward_rows)*

                    #(#reverse_rows)*
                }
            })
        }
    }
}

fn variant_not_found(ident: &Ident) -> darling::Error {
    darling::Error::custom(format!("Missing variant `{}`", ident)).with_span(ident)
}

struct RelMatrixItem<'a> {
    rel: &'a RelationshipType,
    is_reversed: bool,
    dest: Result<&'a Variant, proc_macro2::TokenStream>,
}

impl ToTokens for RelMatrixItem<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.dest {
            Ok(dest) => if let Some(ty) = dest.ty() {
                let rel_type = self.rel;
                let rel_name = if self.is_reversed { self.rel.reverse_name() } else { self.rel.name() };
                let dest_name = dest.set_name();
                let method_name = Ident::new(&format!("{}_{}", rel_name, dest_name), dest.set_name().span());
                
                let filter_method_name = Ident::new(if self.is_reversed {"incoming" } else { "outgoing" }, Span::call_site());

                tokens.append_all(quote! {
                    pub fn #method_name(&'a self) -> impl ::std::iter::Iterator<Item = Node<'a, #ty>> {
                        self.collection.graph().peers_matching(
                            ::stix::Object::id(self.data),
                            ::stix::relationship::Filter::#filter_method_name::<#ty>(::stix::RelationshipType::#rel_type),
                        ).map(move |id| self.link(&self.collection.data().#dest_name[id]))
                    }
                })

            },
            Err(ref e) => tokens.append_all(e.clone()),
        }
    }
}

type RelTuple<'a> = (&'a Ident, &'a RelationshipType, &'a Ident);