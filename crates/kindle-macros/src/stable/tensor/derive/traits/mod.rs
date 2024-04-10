use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive(dim_val: usize, name: &syn::Ident, dims: &[TokenStream]) -> TokenStream {
    let clone = derive_clone(dim_val, name, dims);
    quote! {
        #clone
    }
}

fn derive_clone(dim_val: usize, name: &syn::Ident, dims: &[TokenStream]) -> TokenStream {
    let ty_dims = (0..dim_val)
        .map(|i| {
            let ident = syn::Ident::new(&format!("DIM_{}", i), proc_macro2::Span::call_site());
            quote! {#ident}
        })
        .collect::<Vec<_>>();
    quote! {
        impl <
            Backend,
            Device,
            #(#dims),*,
            Kind,
        > Clone for #name <
            Backend,
            Device,
            #(#ty_dims),*,
            Kind,
        > where
            Backend: kindle_burn::tensor::backend::Backend,
            Device: kindle_burn::device::KindleDevice<Backend>,
            Kind: kindle_burn::tensor::TensorKind<Backend>,
        {
            fn clone(&self) -> Self {
                Self {
                    tensor: self.tensor.clone(),
                    _device: std::marker::PhantomData,
                }
            }
        }
    }
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
