use proc_macro2::TokenStream;
use quote::quote;

mod fns;
mod traits;

pub(crate) fn derive(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    impl_generics: &TokenStream,
    where_clause: &TokenStream,
) -> TokenStream {
    let fns = fns::derive(dim_val, name, dims, impl_generics, where_clause);
    let traits = traits::derive(dim_val, name, dims, impl_generics, where_clause);
    quote! {
        #fns
        #traits
    }
}
