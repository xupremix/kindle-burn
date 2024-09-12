use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_narrow(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    let static_narrow_methods = {
        let mut out = vec![];
        for i in 0..dim_val {
            let doc = format!("Narrows the tensor along dimension {i}");
            let mut new_dims = ty_dims.iter().cloned().collect::<Vec<_>>();
            new_dims[i] = quote! { LENGTH };
            let curr_dim = syn::Ident::new(&format!("DIM_{i}"), proc_macro2::Span::call_site());
            out.push(quote! {
                impl <
                    Backend,
                    Device,
                    #(#dims),*,
                    Kind,
                    const START: usize,
                    const LENGTH: usize,
                >   kindle_burn::dimensions::Narrow<#i, START, LENGTH> for #name <
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
                    fn narrow(self) -> Self::Output {
                        <kindle_burn::const_assert::Range as
                            kindle_burn::const_assert::ConstRange<
                                0,
                                #curr_dim,
                                START,
                                LENGTH,
                            >
                        >::check();
                        #name {
                            tensor: self.tensor.narrow(#i, START, LENGTH),
                            _device: std::marker::PhantomData,
                        }
                    }
                }
            });
        }
        out
    };
    quote! {
        #(#static_narrow_methods)*
    }
}
