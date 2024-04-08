use proc_macro::TokenStream;
use syn::parse::Parse;

struct Entry<T> {
    name: syn::Ident,
    _eq_token: syn::Token![=],
    val: T,
}

pub(crate) struct TensorDefinition {
    vis: Entry<syn::Visibility>,
    _vis_comma: syn::Token![,],
    dim: Entry<syn::LitInt>,
    _trailing_comma: Option<syn::Token![,]>,
}

impl Parse for TensorDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(TensorDefinition {
            vis: Entry {
                name: input
                    .parse()
                    .map_err(|_| syn::Error::new(input.span(), "Missing `vis` identifier"))?,
                _eq_token: input
                    .parse()
                    .map_err(|_| syn::Error::new(input.span(), "Missing `=` token after `vis`"))?,
                val: input.parse().map_err(|_| {
                    syn::Error::new(input.span(), "Missing visibility value after `vis`")
                })?,
            },
            _vis_comma: input
                .parse()
                .map_err(|_| syn::Error::new(input.span(), "Missing `,` token after `vis`"))?,
            dim: Entry {
                name: input
                    .parse()
                    .map_err(|_| syn::Error::new(input.span(), "Missing `dim` identifier"))?,
                _eq_token: input
                    .parse()
                    .map_err(|_| syn::Error::new(input.span(), "Missing `=` token after `dim`"))?,
                val: input.parse().map_err(|_| {
                    syn::Error::new(input.span(), "Missing dimension value after `dim`")
                })?,
            },
            _trailing_comma: input.parse()?,
        })
    }
}

impl TensorDefinition {
    pub(crate) fn vis(&self) -> &syn::Visibility {
        &self.vis.val
    }

    pub(crate) fn dim(&self) -> &syn::LitInt {
        &self.dim.val
    }

    pub(crate) fn check(&self) -> Option<TokenStream> {
        if self.vis.name != "vis" {
            return Some(
                syn::Error::new_spanned(&self.vis.name, "Missing `vis` identifier")
                    .to_compile_error()
                    .into(),
            );
        }
        if self.dim.name != "dim" {
            return Some(
                syn::Error::new_spanned(&self.dim.name, "Missing `dim` identifier")
                    .to_compile_error()
                    .into(),
            );
        }
        None
    }
}
