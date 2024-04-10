use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_init(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    let transposed_dims = {
        if dim_val < 2 {
            (0..dim_val)
                .map(|i| {
                    let ident =
                        syn::Ident::new(&format!("DIM_{i}"), proc_macro2::Span::call_site());
                    quote! { #ident }
                })
                .collect::<Vec<_>>()
        } else {
            let mut transposed_dims = vec![];
            (0..dim_val - 2).for_each(|i| {
                let ident = syn::Ident::new(&format!("DIM_{i}"), proc_macro2::Span::call_site());
                transposed_dims.push(quote! { #ident });
            });
            (dim_val - 2..dim_val).rev().for_each(|i| {
                let ident = syn::Ident::new(&format!("DIM_{i}"), proc_macro2::Span::call_site());
                transposed_dims.push(quote! { #ident });
            });
            transposed_dims
        }
    };
    // let swap_static_dims = {
    //     let mut out = vec![];
    //     for i in 0..dim_val {
    //         for j in i..dim_val {
    //             if i == j {
    //                 continue;
    //             }
    //             out.push(quote! {
    //                 impl kindle_burn::dimensions::Swappable<#i, #j> for Swap<#i, #j> {}
    //             });
    //         }
    //     }
    //     out
    // };
    // let swap_dims_static_methods = {
    //     let mut out = vec![];
    //     for i in 0..dim_val {
    //         for j in i..dim_val {
    //             let doc = format!("Swaps dimensions {i} and {j} of the tensor.");
    //             let mut new_dims = ty_dims.iter().collect::<Vec<_>>();
    //             new_dims.swap(i, j);
    //             out.push(quote! {
    //                 #[doc = #doc]
    //                 pub fn swap_dims<S>(self) -> #name <
    //                     Backend,
    //                     Device,
    //                     #(#new_dims),*,
    //                     Kind,
    //                 > where S: kindle_burn::dimensions::Swappable<#i, #j> {
    //                     #name {
    //                         tensor: self.tensor.swap_dims(#i, #j),
    //                         _device: std::marker::PhantomData,
    //                     }
    //                 }
    //             });
    //         }
    //     }
    //     out
    // };
    quote! {
        // struct Swap<const D1: usize, const D2: usize>;
        // #(#swap_static_dims)*

        impl <
            Backend,
            Device,
            #(#dims),*,
            Kind,
        > #name <
            Backend,
            Device,
            #(#ty_dims),*,
            Kind,
        > where
            Backend: kindle_burn::tensor::backend::Backend,
            Device: kindle_burn::device::KindleDevice<Backend>,
            Kind: kindle_burn::tensor::BasicOps<Backend>,
        {
            /// Creates an Empty tensor.
            pub fn empty() -> Self {
                Self {
                    tensor: kindle_burn::tensor::Tensor::empty(
                        [#(#ty_dims),*],
                        &Device::to_device(),
                    ),
                    _device: std::marker::PhantomData,
                }
            }

            /// Returns the shape of the tensor.
            pub fn shape(&self) -> kindle_burn::tensor::Shape<#dim_val> {
                self.tensor.shape()
            }

            /// Returns the dimensions of the tensor.
            pub fn dims(&self) -> [usize; #dim_val] {
                self.tensor.dims()
            }

            /// Transposes the tensor.
            pub fn transpose(self) -> #name <
                Backend,
                Device,
                #(#transposed_dims),*,
                Kind,
            > {
                #name {
                    tensor: self.tensor.transpose(),
                    _device: std::marker::PhantomData,
                }
            }
            //
            // #(#swap_dims_static_methods)*
        }
    }
}
