use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_init(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
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
            /// Creates an Empty tensor.
            pub fn empty() -> Self {
                Self {
                    tensor: kindle_burn::tensor::Tensor::empty(
                        [#(#ty_dims),*],
                        &Device::to_device(),
                    ),
                    _device: std::marker::PhantomData,
                }
            }
        }
    }
}
