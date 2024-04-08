use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    impl_generics: &TokenStream,
    where_clause: &TokenStream,
) -> TokenStream {
    quote! {}
}

/*
add(E: ElementConversion)
add(K: Numeric)
autodiff
bitxor
clone
clone_from
debug
deserialize
display
div(E)
div(K)
from
from for Param with record (float)
from for Param with record (int)
from for Param with record (bool)
module for tensor
mul(E)
mul(K)
neg
record(float)
record(bool)
record(int)
serialize
sub(E)
sub(K)
*/
