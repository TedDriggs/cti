use std::borrow::Cow;

use darling::{
    ast::{Data, Fields},
    FromDeriveInput, FromVariant,
};
use heck::SnakeCase;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Generics, Ident, Type, Visibility};

#[derive(FromDeriveInput)]
#[darling(attributes(stix), supports(enum_newtype))]
pub struct Collection {
    ident: Ident,
    vis: Visibility,
    generics: Generics,
    data: Data<Variant, ()>,
}

impl ToTokens for Collection {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let vis = &self.vis;
        let (_, ty_generics, where_clause) = self.generics.split_for_impl();
        let builder_match_arms = self
            .data
            .as_ref()
            .map_enum_variants(|v| BuilderMatchArm::new(ident, v))
            .take_enum()
            .unwrap();
        let store_fields = self
            .data
            .as_ref()
            .map_enum_variants(FieldDeclaration)
            .take_enum()
            .unwrap();

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
            }

            impl CollectionBuilder {
                pub fn len(&self) -> usize {
                    1
                }
            }

            struct Indexed<'a> {
                object_indices: ::std::collections::HashMap<&'a ::stix::Id, ::stix::export::petgraph::NodeIndex>,
                graph: ::stix::export::petgraph::Graph<&'a ::stix::Id, &'a ::stix::Relationship>,
            }

            impl<'a> From<&'a CollectionBuilder> for Indexed<'a> {
                fn from(value: &'a CollectionBuilder) -> Self {
                    let mut object_indices = ::std::collections::HashMap::new();
                    let mut graph = ::stix::export::petgraph::Graph::new();

                    for relationship in value.relationships.values() {
                        let source_idx = *object_indices
                            .entry(&relationship.source_ref)
                            .or_insert_with(|| graph.add_node(&relationship.source_ref));

                        let target_idx = *object_indices
                            .entry(&relationship.target_ref)
                            .or_insert_with(|| graph.add_node(&relationship.target_ref));

                        graph.add_edge(source_idx, target_idx, relationship);
                    }

                    Self { object_indices, graph }
                }
            }

            ::stix::export::sync_once_self_cell!(CollectionData, CollectionBuilder, Indexed<'_>);

            #vis struct Collection {
                data: CollectionData
            }
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
        }
    }
}

pub struct DomainObject {
    set_name: Ident,
    type_name: Ident,
    ty: Type,
}

#[derive(FromVariant)]
#[darling(attributes(stix))]
pub struct Variant {
    ident: Ident,
    fields: Fields<Type>,
    #[darling(default)]
    set_name: Option<Ident>,
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
