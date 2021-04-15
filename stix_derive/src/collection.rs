use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use darling::{
    ast::{Data, Fields, Style},
    FromDeriveInput, FromVariant,
};
use heck::SnakeCase;
use proc_macro2::Span;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Generics, Ident, Type, Visibility};

use crate::{relationship::RelationshipType, Relationship, StixCratePath};

#[derive(FromDeriveInput)]
#[darling(attributes(stix), supports(enum_newtype))]
pub struct Collection {
    ident: Ident,
    vis: Visibility,
    generics: Generics,
    data: Data<Variant, ()>,
    /// If set, indicates the macro is being invoked in the `stix` crate.
    #[darling(default)]
    core: StixCratePath,
}

impl Collection {
    /// Convenience function for mapping over all the resource types this declaration supports
    fn variants_as<'a, T>(&'a self, map: impl Fn(LinkedVariant<'a>) -> T) -> Vec<T> {
        self.data
            .as_ref()
            .map_enum_variants(|v| {
                map(LinkedVariant {
                    parent: self,
                    variant: v,
                })
            })
            .take_enum()
            .unwrap()
    }

    fn to_rel_matrix<'a>(&'a self) -> RelMatrix<'a> {
        let objects_by_ident = self
            .variants_as(|v| (&v.variant.ident, v))
            .into_iter()
            .collect::<HashMap<_, _>>();
        let tuples = self
            .variants_as(|v| v.variant.to_rel_tuples())
            .into_iter()
            .flatten()
            .collect();
        RelMatrix {
            objects_by_ident,
            tuples,
        }
    }

    pub(crate) fn stix_crate_path(&self) -> StixCratePath {
        self.core
    }

    /// Add variants to the collection. Variants whose idents are already present will have the relationships from
    /// the new variants added to the existing set.
    pub fn add_variants(&mut self, variants: Vec<Variant>) -> HashSet<Ident> {
        let variant_indices = self
            .data
            .as_ref()
            .take_enum()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(idx, variant)| (&variant.ident, idx))
            .collect::<HashMap<_, _>>();

        let changes_to_make = variants
            .into_iter()
            .map(|variant| (variant_indices.get(&variant.ident).copied(), variant))
            .collect::<Vec<_>>();

        let mut net_new_variants = HashSet::new();

        if let Data::Enum(current_variants) = &mut self.data {
            for (existing_index, new_variant) in changes_to_make {
                if let Some(idx) = existing_index {
                    current_variants[idx].merge(new_variant);
                } else {
                    net_new_variants.insert(new_variant.ident.clone());
                    current_variants.push(new_variant);
                }
            }
        }

        net_new_variants
    }
}

impl ToTokens for Collection {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let vis = &self.vis;
        let (_, ty_generics, where_clause) = self.generics.split_for_impl();
        let builder_match_arms = self.variants_as(BuilderMatchArm);
        let store_fields = self.variants_as(FieldDeclaration);
        let resource_iters = self.variants_as(ResourceIter);
        let ref_impls = self.variants_as(RefImpl);
        let rel_matrix = self.to_rel_matrix();

        let stix = self.core;

        tokens.append_all(quote! {
            #[derive(Default)]
            #vis struct CollectionBuilder #ty_generics #where_clause {
                #(#store_fields),*
            }

            impl CollectionBuilder {
                /// Add a bundle to the collection.
                ///
                /// # ID Collisions
                /// If any of the IDs for objects in `bundle` were already in `self`, they will
                /// be replaced with the new values.
                pub fn add_bundle(&mut self, bundle: #stix::Bundle<#ident>) {
                    for declaration in bundle.objects {
                        match declaration {
                            #(#builder_match_arms),*
                        }
                    }
                }

                /// Finish adding items to the collection and index it for querying.
                pub fn build(self) -> Collection {
                    Collection { data: CollectionData::new(self) }
                }
            }

            impl<'a> Into<#stix::RelationshipGraph<'a>> for &'a CollectionBuilder {
                fn into(self) -> #stix::RelationshipGraph<'a> {
                    self.relationships.values().collect()
                }
            }

            #stix::export::sync_once_self_cell!(CollectionData, CollectionBuilder, #stix::RelationshipGraph<'_>);

            /// An immutable collection of STIX objects. 
            ///
            /// A collection has no special meaning beyond being a set of STIX objects all 
            /// loaded in memory at once.
            ///
            /// # Usage
            /// Create a `Collection` by deserializing JSON into [`Bundle`](stix::Bundle) instances
            /// and then adding those to a `CollectionBuilder`. Once all bundles are loaded, call
            /// [`CollectionBuilder::build`] to finish the collection, which will index the objects
            /// and prepare the collection for querying.
            #vis struct Collection {
                data: CollectionData
            }

            impl Collection {
                /// Create a new [`CollectionBuilder`].
                pub fn builder() -> CollectionBuilder {
                    CollectionBuilder::default()
                }

                /// Get the object identified by `id` if it is present in the collection. This function returns a
                /// [`Node`] which provides access to the object's data and its relationships within the collection.
                pub fn get<'id, 'a: 'id, D>(&'a self, id: &'id #stix::Id) -> Option<Node<'a, D>>
                where
                    Ref<'id, 'a, D>: #stix::Resolve<Output = Node<'a, D>>,
                {
                    // TODO return an Err if D::TYPE != id.object_type()

                    #stix::Resolve::resolve(Ref::<'id, 'a, D> {
                        id,
                        collection: self,
                        object_type: ::std::marker::PhantomData::<D>,
                    })
                }
            }

            /// Accessors for STIX objects in the collection.
            impl Collection {
                #(#resource_iters)*
            }

            impl Collection {
                fn data(&self) -> &CollectionBuilder {
                    self.data.get_owner()
                }

                fn graph(&self) -> &#stix::RelationshipGraph {
                    self.data.get_or_init_dependent()
                }
            }

            impl From<#stix::Bundle<#ident>> for Collection {
                fn from(bundle: #stix::Bundle<#ident>) -> Self {
                    let mut builder = CollectionBuilder::default();
                    builder.add_bundle(bundle);
                    builder.build()
                }
            }

            /// An ID for a resource that may be in the backing collection.
            ///
            /// `Ref` is used to allow exploration of STIX collections when not every object referenced is
            /// present in-memory.
            #[derive(Clone)]
            #vis struct Ref<'id, 'collection: 'id, D> {
                id: &'id #stix::Id,
                collection: &'collection Collection,
                object_type: ::std::marker::PhantomData<D>,
            }

            impl<'id, 'collection: 'id, D> Ref<'id, 'collection, D> {
                /// The ID the `Ref` will look up in the collection.
                pub fn id(&self) -> &'id #stix::Id {
                    self.id
                }
            }

            /// The ID of a STIX object of some type which may be present in the collection.
            impl<'id, 'collection: 'id> Ref<'id, 'collection, #ident> {
                /// Narrow the type of the reference so that it can be resolved to a [`Node`].
                /// This requires knowing the concrete type of the data associated with the ID.
                pub fn downcast<D: #stix::TypedObject>(self) -> Option<Ref<'id, 'collection, D>> {
                    if self.id.object_type() == D::TYPE {
                        Some(Ref {
                            id: self.id,
                            collection: self.collection,
                            object_type: ::std::marker::PhantomData::<D>,
                        })
                    } else {
                        None
                    }
                }
            }

            #(#ref_impls)*

            /// A STIX object in the [`Collection`], exposing the object's data and references
            /// to associated objects in the same collection.
            ///
            /// Relationships are expressed as instance methods, scoped using the concrete type
            /// of the object data, e.g. `Node<'a, IntrusionSet>` exposes `uses_attack_patterns`.
            #[derive(Clone)]
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

                fn create_ref<E>(&self, id: &'a #stix::Id) -> Ref<'a, 'a, E> {
                    Ref {
                        id,
                        collection: self.collection,
                        object_type: ::std::marker::PhantomData::<E>,
                    }
                }

                /// The contents of the STIX object for this node.
                ///
                /// # Usage
                /// This is particularly useful when working with iterators: While the `Node`
                /// might be consumed by a transformation, the underlying data will live as long
                /// as its parent [`Collection`].
                pub fn data(&self) -> &'a D {
                    self.data
                }

                /// The parent collection in which this node's data resides.
                pub fn collection(&self) -> &'a Collection {
                    self.collection
                }
            }

            impl<'a, D> ::std::ops::Deref for Node<'a, D> {
                type Target = D;

                fn deref(&self) -> &Self::Target {
                    self.data
                }
            }

            impl<D: AsRef<#stix::CommonProperties>> AsRef<#stix::CommonProperties> for Node<'_, D> {
                fn as_ref(&self) -> &#stix::CommonProperties {
                    self.data.as_ref()
                }
            }

            #rel_matrix

        });
    }
}

struct BuilderMatchArm<'a>(LinkedVariant<'a>);

impl ToTokens for BuilderMatchArm<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let enum_ident = self.0.declaration_ident();
        let variant_ident = self.0.variant_ident();
        let stix = self.0.stix_crate_path();
        let dest = self.0.variant.set_name();

        if self.0.variant.has_value() {
            tokens.append_all(quote! {
                #enum_ident::#variant_ident(v) => { self.#dest.insert(#stix::Object::id(&v).clone(), v); }
            });
        } else {
            // Known-ignored declaration types are handled to ensure that the enum match remains exhaustive
            tokens.append_all(quote! {
                #enum_ident::#variant_ident => {}
            });
        }
    }
}

#[derive(Clone, FromVariant)]
#[darling(attributes(stix))]
pub struct Variant {
    pub ident: Ident,
    fields: Fields<Type>,
    #[darling(default)]
    set_name: Option<Ident>,
    #[darling(default, multiple)]
    rel: Vec<Relationship>,
}

impl Variant {
    pub fn new(ident: Ident, ty: Type, relationships: Vec<Relationship>) -> Self {
        Self {
            ident,
            fields: Fields::new(Style::Tuple, vec![ty]),
            set_name: None,
            rel: relationships,
        }
    }

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
        self.rel
            .iter()
            .map(move |rel| (&self.ident, &rel.rel, &rel.to))
    }

    fn merge(&mut self, other: Variant) {
        self.rel.extend(other.rel);
    }
}

#[derive(Clone, Copy)]
pub struct LinkedVariant<'a> {
    variant: &'a Variant,
    parent: &'a Collection,
}

impl<'a> LinkedVariant<'a> {
    fn declaration_ident(&'a self) -> &'a Ident {
        &self.parent.ident
    }

    fn variant_ident(&'a self) -> &'a Ident {
        &self.variant.ident
    }

    fn stix_crate_path(&self) -> StixCratePath {
        self.parent.core
    }

    fn set_name(&'a self) -> Cow<'a, Ident> {
        self.variant.set_name()
    }

    fn ty(&'a self) -> Option<&'a Type> {
        self.variant.ty()
    }
}

struct FieldDeclaration<'a>(LinkedVariant<'a>);

impl ToTokens for FieldDeclaration<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let var_name = self.0.set_name();
        let stix = self.0.stix_crate_path();
        if let Some(ty) = self.0.ty() {
            tokens.append_all(quote!(#var_name: #stix::export::IndexMap<#stix::Id, #ty>));
        }
    }
}

struct ResourceIter<'a>(LinkedVariant<'a>);

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

struct RefImpl<'a>(LinkedVariant<'a>);

impl ToTokens for RefImpl<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(ty) = self.0.ty() {
            let set_name = self.0.set_name();
            let stix = self.0.stix_crate_path();

            tokens.append_all(quote! {
                impl<'id, 'collection: 'id> Ref<'id, 'collection, #ty> {
                    pub fn resolve(self) -> Option<Node<'collection, #ty>> {
                        let Self { id, collection, .. } = self;
                        Some(Node {
                            data: collection.data().#set_name.get(id)?,
                            collection,
                        })
                    }
                }

                impl<'id, 'collection: 'id> #stix::Resolve for Ref<'id, 'collection, #ty> {
                    type Output = Node<'collection, #ty>;

                    fn resolve(self) -> Option<Self::Output> {
                        Ref::<'id, 'collection, #ty>::resolve(self)
                    }
                }
            });
        }
    }
}

struct RelMatrix<'a> {
    objects_by_ident: HashMap<&'a Ident, LinkedVariant<'a>>,
    tuples: Vec<RelTuple<'a>>,
}

impl ToTokens for RelMatrix<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for (ident, variant) in &self.objects_by_ident {
            let ident_str = ident.to_string();

            let forward_rows = self
                .tuples
                .iter()
                .filter(|tup| tup.0 == &ident_str)
                .map(|i| RelMatrixItem {
                    rel: &i.1,
                    is_reversed: false,
                    dest: self
                        .objects_by_ident
                        .get(i.2)
                        .copied()
                        .ok_or_else(|| variant_not_found(i.2).write_errors()),
                })
                .collect::<Vec<_>>();

            let reverse_rows = self
                .tuples
                .iter()
                .filter(|tup| tup.2 == &ident_str)
                .map(|i| RelMatrixItem {
                    rel: &i.1,
                    is_reversed: true,
                    dest: self
                        .objects_by_ident
                        .get(i.0)
                        .copied()
                        .ok_or_else(|| variant_not_found(i.0).write_errors()),
                })
                .collect::<Vec<_>>();

            let ty = variant.ty();
            if !forward_rows.is_empty() || !reverse_rows.is_empty() {
                tokens.append_all(quote! {
                    impl<'a> Node<'a, #ty> {
                        #(#forward_rows)*

                        #(#reverse_rows)*
                    }
                });
            }

            // Generate AsRef on the concrete type to avoid creating impl collisions with the
            // CommonProperties AsRef.
            tokens.append_all(quote! {
                impl<'a> ::std::convert::AsRef<#ty> for Node<'a, #ty> {
                    fn as_ref(&self) -> &#ty {
                        self.data()
                    }
                }
            });
        }
    }
}

fn variant_not_found(ident: &Ident) -> darling::Error {
    darling::Error::custom(format!("Missing variant `{}`", ident)).with_span(ident)
}

struct RelMatrixItem<'a> {
    rel: &'a RelationshipType,
    is_reversed: bool,
    dest: Result<LinkedVariant<'a>, proc_macro2::TokenStream>,
}

impl ToTokens for RelMatrixItem<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.dest {
            Ok(dest) => {
                if let Some(ty) = dest.ty() {
                    let stix = dest.stix_crate_path();
                    let rel_type = self.rel;
                    let rel_name = if self.is_reversed {
                        self.rel.reverse_name()
                    } else {
                        self.rel.name()
                    };
                    let dest_name = dest.set_name();
                    let method_name = Ident::new(
                        &format!("{}_{}", rel_name, dest_name),
                        dest.set_name().span(),
                    );

                    let filter_method_name = Ident::new(
                        if self.is_reversed {
                            "incoming"
                        } else {
                            "outgoing"
                        },
                        Span::call_site(),
                    );

                    tokens.append_all(quote! {
                    pub fn #method_name(&'a self) -> impl ::std::iter::Iterator<Item = Ref<'a, 'a, #ty>> {
                        self.collection.graph().peers_matching(
                            #stix::Object::id(self.data),
                            #stix::relationship::Filter::#filter_method_name::<#ty>(#stix::RelationshipType::#rel_type),
                        ).map(move |id| self.create_ref::<#ty>(id))
                    }
                })
                }
            }
            Err(ref e) => tokens.append_all(e.clone()),
        }
    }
}

type RelTuple<'a> = (&'a Ident, &'a RelationshipType, &'a Ident);
