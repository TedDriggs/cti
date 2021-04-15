use darling::FromMeta;
use syn::{DeriveInput, Ident, LitStr};

pub fn custom_properties(meta: syn::Meta, item: DeriveInput) -> darling::Result<DeriveInput> {
    // XXX without this, darling starts one layer too deep.
    let attr: syn::Attribute = syn::parse_quote!(#[attr(#meta)]);
    let opts = Opts::from_meta(&attr.parse_meta()?)?;
    transform(opts, item)
}

#[derive(FromMeta)]
struct Opts {
    namespace: Ident,
}

fn transform(opts: Opts, mut input: DeriveInput) -> darling::Result<DeriveInput> {
    input.attrs.retain(|attr| !attr.path.is_ident("stix"));
    if let syn::Data::Struct(data) = &mut input.data {
        for field in data.fields.iter_mut() {
            let field_ident = field.ident.as_ref().unwrap();
            let serde_name = LitStr::new(
                &format!(
                    "x_{}_{}",
                    opts.namespace.to_string(),
                    field_ident.to_string()
                ),
                field_ident.span(),
            );

            field.attrs.retain(|attr| !attr.path.is_ident("stix"));
            field
                .attrs
                .push(syn::parse_quote!(#[serde(rename = #serde_name)]));
        }

        Ok(input)
    } else {
        Err(darling::Error::unsupported_shape("enum"))
    }
}
