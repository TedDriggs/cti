use darling::FromDeriveInput;
use quote::quote;

mod collection;
mod plurals;
mod typed_object;

use collection::Collection;
pub(crate) use plurals::pluralize;
use typed_object::TypedObject;

#[proc_macro_derive(TypedObject, attributes(typed_object))]
pub fn derive_typed_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = match TypedObject::from_derive_input(&syn::parse_macro_input!(input)) {
        Ok(v) => quote!(#v),
        Err(e) => e.write_errors(),
    };

    tokens.into()
}

#[proc_macro_derive(Collection, attributes(collection))]
pub fn derive_collection(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = match Collection::from_derive_input(&syn::parse_macro_input!(input)) {
        Ok(v) => quote!(#v),
        Err(e) => e.write_errors(),
    };

    tokens.into()
}
