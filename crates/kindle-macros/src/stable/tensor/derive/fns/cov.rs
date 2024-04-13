use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_cov(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    let cov_static_methods = {
        let mut out = vec![];
        for i in 0..dim_val {
            let mut out_dims = ty_dims.iter().cloned().collect::<Vec<_>>();
            out_dims.swap(i, 0);
            out_dims[dim_val - 2] = out_dims[dim_val - 1].clone();
            out.push(quote! {
                impl<
                    Backend,
                    Device,
                    #(#dims),*,
                > kindle_burn::dimensions::Covariance::<{ #i }> for #name <
                    Backend,
                    Device,
                    #(#ty_dims),*,
                    kindle_burn::tensor::Float,
                >
                where
                    Backend: kindle_burn::tensor::backend::Backend,
                    Device: kindle_burn::device::KindleDevice<Backend>,
                {
                    type Output = #name <
                        Backend,
                        Device,
                        #(#out_dims),*,
                        kindle_burn::tensor::Float,
                    >;
                    fn cov(self, correction_factor: usize) -> Self::Output {
                        #name {
                            tensor: self.tensor.cov(#i, correction_factor),
                            _device: std::marker::PhantomData,
                        }
                    }
                }
            });
        }
        out
    };
    quote! {
        #(#cov_static_methods)*
    }
}
