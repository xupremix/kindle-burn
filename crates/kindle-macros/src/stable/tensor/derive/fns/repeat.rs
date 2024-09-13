use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_repeat(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    let repeat_static_methods = {
        let mut out = vec![];
        for i in 0..dim_val {
            let doc = format!("Repeats the tensor along dimension {i}, N times");
            let mut new_const_dims = (0..dim_val)
                .map(|j| {
                    let ident =
                        syn::Ident::new(&format!("DIM_{j}"), proc_macro2::Span::call_site());
                    quote! { const #ident: usize }
                })
                .collect::<Vec<_>>();
            _ = new_const_dims.remove(i);
            let mut new_ty_dims = ty_dims.to_vec();
            new_ty_dims[i] = quote! { 1 };
            let mut new_dims = ty_dims.to_vec();
            new_dims[i] = quote! { N };
            out.push(quote! {
                impl <
                    Backend,
                    Device,
                    #(#new_const_dims),*,
                    Kind,
                    const N: usize,
                >   kindle_burn::dimensions::Repeat<#i, N> for #name <
                    Backend,
                    Device,
                    #(#new_ty_dims),*,
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
                    fn repeat(self) -> Self::Output {
                        #name {
                            tensor: self.tensor.repeat(#i, N),
                            _device: std::marker::PhantomData,
                        }
                    }
                }
            });
        }
        out
    };
    quote! {
        #(#repeat_static_methods)*
    }
}
