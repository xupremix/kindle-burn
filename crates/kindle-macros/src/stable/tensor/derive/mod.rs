use proc_macro2::TokenStream;
use quote::quote;

mod fns;
mod traits;

pub(crate) fn derive(dim_val: usize, name: &syn::Ident, dims: &Vec<TokenStream>) -> TokenStream {
    let fns = fns::derive(dim_val, name, dims);
    let traits = traits::derive(dim_val, name, dims);
    quote! {
        #fns
        #traits
    }
}
