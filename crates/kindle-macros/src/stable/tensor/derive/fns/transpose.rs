use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_transpose(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    let transposed_dims = {
        if dim_val < 2 {
            (0..dim_val)
                .map(|i| {
                    let ident =
                        syn::Ident::new(&format!("DIM_{i}"), proc_macro2::Span::call_site());
                    quote! { #ident }
                })
                .collect::<Vec<_>>()
        } else {
            let mut transposed_dims = vec![];
            (0..dim_val - 2).for_each(|i| {
                let ident = syn::Ident::new(&format!("DIM_{i}"), proc_macro2::Span::call_site());
                transposed_dims.push(quote! { #ident });
            });
            (dim_val - 2..dim_val).rev().for_each(|i| {
                let ident = syn::Ident::new(&format!("DIM_{i}"), proc_macro2::Span::call_site());
                transposed_dims.push(quote! { #ident });
            });
            transposed_dims
        }
    };
    quote! {

        impl <
            Backend,
            Device,
            #(#dims),*,
            Kind,
        > #name <
            Backend,
            Device,
            #(#ty_dims),*,
            Kind,
        > where
            Backend: kindle_burn::tensor::backend::Backend,
            Device: kindle_burn::device::KindleDevice<Backend>,
            Kind: kindle_burn::tensor::BasicOps<Backend>,
        {
            /// Transposes the tensor.
            pub fn transpose(self) -> #name <
                Backend,
                Device,
                #(#transposed_dims),*,
                Kind,
            > {
                #name {
                    tensor: self.tensor.transpose(),
                    _device: std::marker::PhantomData,
                }
            }
        }
    }
}
