use darling::FromMeta;
use quote::{quote, ToTokens, TokenStreamExt};

/// Path the macro should use to access `::stix`. When a proc-macro is invoked
/// inside the `stix` crate to build the standard objects and collections,
/// `crate` must be used or the path will not resolve.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StixCratePath(bool);

impl FromMeta for StixCratePath {
    fn from_word() -> darling::Result<Self> {
        Ok(Self(true))
    }
}

impl ToTokens for StixCratePath {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(if self.0 {
            quote!(crate)
        } else {
            quote!(::stix)
        })
    }
}
