use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_one_hot(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    let d_m_1_1s = (0..dim_val - 1).map(|_| 1usize).collect::<Vec<_>>();
    quote! {
        impl <
            Backend,
            Device,
            const CLASSES: usize,
        > #name <
            Backend,
            Device,
            #(#d_m_1_1s),*,
            CLASSES,
            kindle_burn::tensor::Float,
        >
        where
            Backend: kindle_burn::tensor::backend::Backend,
            Device: kindle_burn::device::KindleDevice<Backend>,
        {
            /// Creates a one hot encoded tensor at the given index
            pub fn one_hot<const IDX: usize>() -> Self {
                _ = <
                    kindle_burn::const_assert::Value as
                    kindle_burn::const_assert::ConstValueBetween<
                        IDX,
                        0,
                        CLASSES,
                    >
                >::VALID;
                Self {
                    tensor: kindle_burn::tensor::Tensor::<
                        Backend,
                        #dim_val,
                        kindle_burn::tensor::Float,
                    >::one_hot(IDX, CLASSES, &Device::to_device()),
                    _device: std::marker::PhantomData,
                }
            }

        }
    }
}
