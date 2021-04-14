use std::{borrow::Cow, collections::HashSet, convert::TryFrom};

use darling::FromDeriveInput;
use heck::{KebabCase, ShoutySnakeCase};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{bracketed, parse::Parse, punctuated::Punctuated, Attribute, Ident, LitStr, Token};

use crate::StixCratePath;

struct ParseTerm {
    ident: Ident,
    literal: Option<LitStr>,
}

impl Parse for ParseTerm {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let lookahead = input.lookahead1();
        let literal = if lookahead.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self { ident, literal })
    }
}

pub struct Item {
    attrs: Vec<Attribute>,
    vocab_ident: Ident,
    terms: Punctuated<ParseTerm, Token![,]>,
}

impl Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            vocab_ident: input.parse()?,
            terms: {
                input.parse::<Token![=]>()?;
                let content;
                bracketed!(content in input);
                content.parse_terminated(ParseTerm::parse)?
            },
        })
    }
}

pub struct Vocab {
    attrs: Vec<Attribute>,
    ident: Ident,
    terms: Vec<KnownTerm>,
    opts: Opts,
}

impl Vocab {
    fn type_name(&self) -> Cow<'_, LitStr> {
        self.opts
            .name
            .as_ref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| {
                Cow::Owned(LitStr::new(
                    &format!("{}-ov", self.ident.to_string().to_kebab_case()),
                    self.ident.span(),
                ))
            })
    }

    fn terms_as<'a, T>(&'a self, map: impl Fn(&'a KnownTerm) -> T) -> Vec<T> {
        self.terms.iter().map(map).collect()
    }
}

impl TryFrom<Item> for Vocab {
    type Error = darling::Error;

    fn try_from(value: Item) -> Result<Self, Self::Error> {
        let mut errors = vec![];
        let mut seen_idents = HashSet::new();
        for term in &value.terms {
            let ident = &term.ident;
            if !seen_idents.insert(ident) {
                errors.push(
                    darling::Error::custom(format!("Duplicate term `{}`", ident)).with_span(ident),
                );
            }
        }

        let (meaningful, passthrough): (Vec<_>, Vec<_>) = value
            .attrs
            .into_iter()
            .partition(|attr| attr.path.is_ident("vocabulary"));

        let opts = if meaningful.is_empty() {
            Opts::default()
        } else {
            Opts::from_derive_input(&syn::parse_quote!(#(#meaningful)* struct Temp;))?
        };

        if !errors.is_empty() {
            Err(darling::Error::multiple(errors))
        } else {
            Ok(Self {
                attrs: passthrough,
                ident: value.vocab_ident,
                terms: value.terms.into_iter().map(KnownTerm::from).collect(),
                opts,
            })
        }
    }
}

impl ToTokens for Vocab {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let stix = self.opts.core;
        let ident = &self.ident;
        let attrs = &self.attrs;
        let type_name = self.type_name();

        let const_declarations = self.terms_as(|term| LinkedTerm { term });
        let literals = self.terms_as(|t| &t.literal_value);

        tokens.append_all(quote! {
            #(#attrs)*
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ::serde::Deserialize, ::serde::Serialize)]
            pub struct #ident(::std::borrow::Cow<'static, str>);

            impl #ident {
                #(#const_declarations)*
            }

            impl #stix::vocab::Vocabulary for #ident {
                const TYPE: &'static str = #type_name;

                fn is_known_value(&self) -> bool {
                    #(self == #literals) ||*
                }
            }

            impl ::std::fmt::Display for #ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl ::std::convert::AsRef<str> for #ident {
                fn as_ref(&self) -> &str {
                    &self.0
                }
            }

            impl ::std::cmp::PartialEq<str> for #ident {
                fn eq(&self, rhs: &str) -> bool {
                    self.0 == rhs
                }
            }

            impl ::std::convert::From<String> for #ident {
                fn from(s: String) -> Self {
                    Self(::std::borrow::Cow::Owned(s))
                }
            }

            impl ::std::convert::From<&'static str> for #ident {
                fn from(s: &'static str) -> Self {
                    Self(::std::borrow::Cow::Borrowed(s))
                }
            }
        });
    }
}

struct KnownTerm {
    const_ident: Ident,
    literal_value: LitStr,
    has_explicit_literal: bool,
}

impl From<ParseTerm> for KnownTerm {
    fn from(term: ParseTerm) -> Self {
        let ParseTerm { ident, literal } = term;
        let has_explicit_literal = literal.is_some();
        Self {
            const_ident: Ident::new(&ident.to_string().to_shouty_snake_case(), ident.span()),
            has_explicit_literal,
            literal_value: literal
                .unwrap_or_else(|| LitStr::new(&ident.to_string().to_kebab_case(), ident.span())),
        }
    }
}

struct LinkedTerm<'a> {
    term: &'a KnownTerm,
}

impl ToTokens for LinkedTerm<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let const_ident = &self.term.const_ident;
        let term_value = &self.term.literal_value;

        let docs = if self.term.has_explicit_literal {
            let doc_str = format!(r#"Value is `"{}"`"#, term_value.value());
            Some(quote!(#[doc = #doc_str]))
        } else {
            None
        };

        tokens.append_all(quote! {
            #docs
            pub const #const_ident: Self = Self(::std::borrow::Cow::Borrowed(#term_value));
        });
    }
}

#[derive(Default, FromDeriveInput)]
#[darling(default, attributes(vocabulary))]
struct Opts {
    /// Flag indicating the macro is being called inside the `stix` crate.
    core: StixCratePath,
    /// If set, the name that will be used by `Vocabulary::TYPE`. By default, this is generated from
    /// the vocabulary's ident.
    name: Option<LitStr>,
}
