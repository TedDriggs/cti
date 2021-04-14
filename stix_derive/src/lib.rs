use std::convert::TryFrom;

use darling::FromDeriveInput;
use quote::quote;

mod collection;
mod plurals;
mod relationship;
mod stix_crate_path;
mod typed_object;
mod vocab;

use collection::Collection;
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
/// ```rust,no_run
/// # use crate::vocabulary;
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
