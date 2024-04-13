use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_var(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    let var_static_methods = {
        let mut out = vec![];
        for i in 0..dim_val {
            let mut out_dims = ty_dims.iter().cloned().collect::<Vec<_>>();
            out_dims[i] = quote! { 1usize };
            out.push(quote! {
                impl<
                    Backend,
                    Device,
                    #(#dims),*,
                > kindle_burn::dimensions::Variance::<{ #i }> for #name <
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
                    fn var(self) -> Self::Output {
                        #name {
                            tensor: self.tensor.var(#i),
                            _device: std::marker::PhantomData,
                        }
                    }
                    fn var_mean(self) -> (Self::Output, Self::Output) {
                        let (var, mean) = self.tensor.var_mean(#i);
                        (
                            #name {
                                tensor: var,
                                _device: std::marker::PhantomData,
                            },
                            #name {
                                tensor: mean,
                                _device: std::marker::PhantomData,
                            },
                        )
                    }
                    fn var_mean_bias(self) -> (Self::Output, Self::Output) {
                        let (var, mean) = self.tensor.var_mean_bias(#i);
                        (
                            #name {
                                tensor: var,
                                _device: std::marker::PhantomData,
                            },
                            #name {
                                tensor: mean,
                                _device: std::marker::PhantomData,
                            },
                        )
                    }
                }
            });
        }
        out
    };
    quote! {
        #(#var_static_methods)*
    }
}
