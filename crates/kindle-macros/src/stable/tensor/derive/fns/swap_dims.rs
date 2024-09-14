use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_switch_dims(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    let swap_dims_static_methods = {
        let mut out = vec![];
        for i in 0..dim_val {
            for j in i + 1..dim_val {
                let doc = format!("Swaps dimensions {i} and {j} of the tensor.");
                let mut new_dims = ty_dims.to_vec();
                new_dims.swap(i, j);
                out.push(quote! {
                    impl <
                        Backend,
                        Device,
                        #(#dims),*,
                        Kind,
                    >   kindle_burn::dimensions::Swap<#i, #j> for #name <
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
                            #(#new_dims),*,
                            Kind,
                        >;
                        #[doc = #doc]
                        fn swap_dims(self) -> Self::Output {
                            #name {
                                tensor: self.tensor.swap_dims(#i, #j),
                                _device: std::marker::PhantomData,
                            }
                        }
                    }
                });
            }
        }
        out
    };
    quote! {
        #(#swap_dims_static_methods)*
    }
}
