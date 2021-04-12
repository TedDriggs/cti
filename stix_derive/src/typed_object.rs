use darling::FromDeriveInput;
use heck::KebabCase;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{Generics, Ident};

#[derive(FromDeriveInput)]
#[darling(attributes(typed_object))]
pub struct TypedObject {
    ident: Ident,
    generics: Generics,
    #[darling(default)]
    name: Option<String>,
}

impl quote::ToTokens for TypedObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();
        let name_str = self
            .name
            .clone()
            .unwrap_or_else(|| ident.to_string().to_kebab_case());

        tokens.append_all(quote! {
            impl #impl_generics ::stix::TypedObject for #ident #type_generics #where_clause {
                const TYPE: &'static str = #name_str;
            }
        });
    }
}
