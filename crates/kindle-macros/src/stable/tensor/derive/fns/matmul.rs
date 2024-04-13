use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_matmul(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    let new_dim = syn::Ident::new(&format!("DIM_{dim_val}"), proc_macro2::Span::call_site());
    let const_new_dim = quote! { const #new_dim: usize };
    let other_dims = {
        let mut out = ty_dims
            .iter()
            .take(dim_val - 2)
            .cloned()
            .collect::<Vec<_>>();
        out.push(ty_dims[dim_val - 1].clone());
        out.push(quote! { #new_dim });
        out
    };
    let mut new_dims = other_dims.clone();
    new_dims[dim_val - 1] = quote! { #new_dim };
    quote! {
        impl <
            Backend,
            Device,
            #(#dims),*,
        >   #name <
            Backend,
            Device,
            #(#ty_dims),*,
            kindle_burn::tensor::Float,
        >
        where
            Backend: kindle_burn::tensor::backend::Backend,
            Device: kindle_burn::device::KindleDevice<Backend>,
        {
            /// Performs matrix multiplication
            /// NOTE: (follows the pytorch - numpy dimension convention)
            /// Es.
            /// DIMS:  <D0, D1, D2, D3>
            /// OTHER: <D0, D1, D3, D4>
            /// RIS:   <D0, D1, D2, D4>
            pub fn matmul<#const_new_dim>(
                self,
                other: #name<
                    Backend,
                    Device,
                    #(#other_dims),*,
                    kindle_burn::tensor::Float,
                >
            ) -> #name <
                Backend,
                Device,
                #(#new_dims),*,
                kindle_burn::tensor::Float,
            >{
                #name {
                    tensor: self.tensor.matmul(other.tensor),
                    _device: std::marker::PhantomData,
                }
            }
        }
    }
}
