use std::convert::TryFrom;

use darling::FromDeriveInput;
use proc_macro2::Span;
use quote::quote;
use standard::standard_objects;
use syn::DeriveInput;

mod collection;
mod plurals;
mod relationship;
mod standard;
mod stix_crate_path;
mod typed_object;
mod vocab;

pub(crate) use collection::{Collection, Variant};
pub(crate) use plurals::pluralize;
pub(crate) use relationship::Relationship;
pub(crate) use stix_crate_path::StixCratePath;
use typed_object::TypedObject;

#[proc_macro_derive(TypedObject, attributes(typed_object))]
pub fn derive_typed_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = match TypedObject::from_derive_input(&syn::parse_macro_input!(input)) {
        Ok(v) => quote!(#v),
        Err(e) => e.write_errors(),
    };

    tokens.into()
}

#[proc_macro_derive(Collection, attributes(stix))]
pub fn derive_collection(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = match Collection::from_derive_input(&syn::parse_macro_input!(input)) {
        Ok(v) => quote!(#v),
        Err(e) => e.write_errors(),
    };

    tokens.into()
}

/// Define an open vocabulary in STIX. Open vocabularies have a set of standard-defined values
/// and allow arbitrary additional values.
///
/// # Example
/// ```rust,ignore
/// # // This test is ignored because the generated code depends on the `stix` crate
/// # // and this test can't access that.
/// vocabulary!(
///     /// Doc comments carry through, as do other attributes.
///     #[cfg(not(windows))]
///     ImplementationLanguage = [
///         applescript,
///         bash,
///         c_plus_plus = "c++",
///         c_sharp = "c#",
///         x86_32,
///         x86_64
///     ]
/// );
/// ```
#[proc_macro]
pub fn vocabulary(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as vocab::Item);
    let tokens = match vocab::Vocab::try_from(input) {
        Ok(v) => quote!(#v),
        Err(e) => e.write_errors(),
    };

    tokens.into()
}

/// Transform an enum into an extension of the STIX standard's object types.
#[proc_macro_attribute]
pub fn extension(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input: DeriveInput = syn::parse_macro_input!(item);

    // Start the collection with explicit user input. This will cause us to prefer spans
    // into user input over the generic call-site span, making errors better in most
    // cases.
    let mut di = match Collection::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let standard_variants = standard_objects(di.stix_crate_path());
    let variants_to_insert = di.add_variants(standard_variants.clone());

    // Drop STIX attributes; they've been consumed by the `Collection` invocation
    // and we don't want to leak them into the emitted code.
    input.attrs.retain(|attr| !attr.path.is_ident("stix"));

    if let syn::Data::Enum(data) = &mut input.data {
        // Drop STIX attributes; they've been consumed by the `Collection` invocation
        // and we don't want to leak them into the emitted code.
        for variant in &mut data.variants {
            variant.attrs.retain(|attr| !attr.path.is_ident("stix"));
        }

        for variant in standard_variants {
            if !variants_to_insert.contains(&variant.ident) {
                continue;
            }

            let ty = variant.ty().unwrap().clone();
            data.variants.push(syn::Variant {
                attrs: vec![],
                discriminant: None,
                ident: variant.ident,
                fields: syn::Fields::Unnamed(syn::FieldsUnnamed {
                    paren_token: syn::token::Paren(Span::call_site()),
                    unnamed: {
                        let mut sequence = syn::punctuated::Punctuated::new();
                        sequence.push(syn::Field {
                            attrs: vec![],
                            colon_token: None,
                            ident: None,
                            ty,
                            vis: syn::Visibility::Inherited,
                        });
                        sequence
                    },
                }),
            });
        }
    } else {
        panic!("Structs are not valid macro targets");
    }

    quote!(#input #di).into()
}
