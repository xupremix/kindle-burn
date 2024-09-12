use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_cat(
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
                tensors: &[kindle_burn::tensor::Tensor<
                    Backend,
                    #dim_val,
                    Kind,
                >],
            ) -> Self {
                <kindle_burn::const_assert::Value as
                    kindle_burn::const_assert::ConstValueBetween<
                        DIM, 0, #dim_val
                    >
                >::VALID;

                // check for the sum of dims
                let ty_dim = [#(#ty_dims),*];
                let dim = ty_dim[DIM];
                let sum = tensors.iter().map(|t| t.dims()[DIM]).sum::<usize>();
                assert_eq!(sum, dim, "Wrong dimension sum provided, expected: {sum}, got: {dim}");

                // check for the other dimensions
                for i in 0..#dim_val {
                    if i == DIM {
                        continue;
                    }
                    if tensors.iter().any(|t| t.dims()[i] != ty_dim[i]) {
                        panic!("The other dimensions provided are not equal");
                    }
                }

                Self {
                    tensor: kindle_burn::tensor::Tensor::<
                            Backend,
                            #dim_val,
                            Kind,
                        >::cat(tensors.iter().cloned().collect::<Vec<_>>(), DIM),
                    _device: std::marker::PhantomData,
                }
            }
        }
    }
}
