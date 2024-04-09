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
            // 'dv,
            Backend,
            Device,
            #(#dims),*,
            Kind,
        > #name <
            // 'dv,
            Backend,
            Device,
            #(#ty_dims),*,
            Kind,
        > where
            Backend: kindle_burn::tensor::backend::Backend,
            Device: kindle_burn::device::KindleDevice</*'dv,*/ Backend>,
            Kind: kindle_burn::tensor::BasicOps<Backend>,
        {
            /// Empty tensor.
            pub fn empty() -> Self {
                todo!()
                // Self {
                //     tensor: kindle_burn::tensor::Tensor::empty(
                //         kindle_burn::tensor::Shape::new([#(#ty_dims),*]),
                //         Device::into(),
                //     ),
                //     _device: std::marker::PhantomData,
                // }
            }
        }
    }
}
