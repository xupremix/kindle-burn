use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_cat(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    let new_dim = dim_val + 1;
    quote! {
        impl <
            Backend,
            Device,
            #(#dims),*,
            Kind,
        >   #name <
            Backend,
            Device,
            #(#ty_dims),*,
            Kind,
        >
        where
            Backend: kindle_burn::tensor::backend::Backend,
            Device: kindle_burn::device::KindleDevice<Backend>,
            Kind: kindle_burn::tensor::BasicOps<Backend>,
        {
            /// Concatenates the tensors along the given dimension.
            /// WARNING: This function panics if the provided dimension results are not == to the
            /// resulting sum of the dimensions of the tensors.
            fn cat_unchecked<const DIM: usize>(
                tensors: &[Self],
            ) -> Self {
                _ = <kindle_burn::const_assert::Value as
                    kindle_burn::const_assert::ConstValueBetween<
                        DIM, 0, #new_dim
                    >
                >::validate();
                let tensors = tensors.iter().map(|t| t.tensor.clone()).collect::<Vec<_>>();
                let tensor = kindle_burn::tensor::Tensor::cat(tensors, DIM);

                let provided_dims = [#(#ty_dims),*];
                let dims = tensor.dims();
                if dims.iter().enumerate().any(|(i, d)| *d != provided_dims[i]) {
                    panic!("The provided dimensions are not equal to the resulting dimensions, \
                        Found: {:?}, Expected: {:?}", dims, provided_dims);
                }
                Self {
                    tensor,
                    _device: std::marker::PhantomData,
                }
            }
        }
    }
}
