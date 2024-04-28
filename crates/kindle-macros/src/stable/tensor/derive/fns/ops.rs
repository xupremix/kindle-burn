use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_ops(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    quote! {
        impl<
            Backend,
            Device,
            #(#dims),*,
            Kind,
        > #name <
            Backend,
            Device,
            #(#ty_dims),*,
            Kind,
        >
        where
            Backend: kindle_burn::tensor::backend::Backend,
            Device: kindle_burn::device::KindleDevice<Backend>,
            Kind: kindle_burn::tensor::Numeric<Backend>,
            <Kind as kindle_burn::tensor::BasicOps<Backend>>::Elem: kindle_burn::tensor::Element,
        {
            pub fn add(self, other: Self) -> Self {
                Self {
                    tensor: self.tensor.add(other.tensor),
                    _device: std::marker::PhantomData,
                }
            }
            pub fn add_scalar<Scalar>(self, other: Scalar) -> Self
            where
                Scalar: kindle_burn::prelude::ElementConversion
            {
                Self {
                    tensor: self.tensor.add_scalar(other),
                    _device: std::marker::PhantomData,
                }
            }
        }
    }
}
