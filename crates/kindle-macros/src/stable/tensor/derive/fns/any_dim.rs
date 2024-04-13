use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_any_dim(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    let any_dim_static_methods = {
        let mut out = vec![];
        for i in 0..dim_val {
            let mut out_dims = ty_dims.iter().cloned().collect::<Vec<_>>();
            out_dims[i] = quote! { 1usize };
            out.push(quote! {
                impl<
                    Backend,
                    Device,
                    #(#dims),*,
                    Kind,
                > kindle_burn::dimensions::AnyDim::<{ #i }> for #name <
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
                    type Output = #name <
                        Backend,
                        Device,
                        #(#out_dims),*,
                        kindle_burn::tensor::Bool,
                    >;
                    fn any_dim(self) -> Self::Output {
                        #name {
                            tensor: self.tensor.any_dim(#i),
                            _device: std::marker::PhantomData,
                        }
                    }
                }
            });
        }
        out
    };
    quote! {
        #(#any_dim_static_methods)*
    }
}
