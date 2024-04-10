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
    let swap_dims_static_methods = {
        let mut out = vec![];
        for i in 0..dim_val {
            for j in i + 1..dim_val {
                let doc = format!("Swaps dimensions {i} and {j} of the tensor.");
                let mut new_dims = ty_dims.iter().collect::<Vec<_>>();
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
    let const_ranges = (0..dim_val)
        .map(|i| {
            let ident_range_1 =
                syn::Ident::new(&format!("RANGE_{i}_0"), proc_macro2::Span::call_site());
            let ident_range_2 =
                syn::Ident::new(&format!("RANGE_{i}_1"), proc_macro2::Span::call_site());
            quote! {
                const #ident_range_1: usize,
                const #ident_range_2: usize,
            }
        })
        .collect::<Vec<_>>();
    let ranges_idents = (0..dim_val)
        .map(|i| syn::Ident::new(&format!("RANGE_{i}_1"), proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();
    let ranges = (0..dim_val)
        .map(|i| syn::Ident::new(&format!("range_{i}"), proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();
    let check = ranges
        .iter()
        .zip(ty_dims.iter())
        .enumerate()
        .map(|(i, (range, dim))| {
            let first = syn::Ident::new(&format!("RANGE_{i}_0"), proc_macro2::Span::call_site());
            let second = syn::Ident::new(&format!("RANGE_{i}_1"), proc_macro2::Span::call_site());
            quote! {
                let #range = <
                    kindle_burn::dimensions::Range as
                        kindle_burn::dimensions::ConstRange<
                            0,
                            #dim,
                            #first,
                            #second,
                        >
                    >::new();
            }
        })
        .collect::<Vec<_>>();

    let assign_new_tensor_dims = (0..dim_val)
        .map(|i| {
            let ident = syn::Ident::new(&format!("V_DIM_{i}"), proc_macro2::Span::call_site());
            quote! {
                const #ident: usize,
            }
        })
        .collect::<Vec<_>>();
    let assign_const_ranges = (0..dim_val)
        .map(|i| {
            let ident_range_1 =
                syn::Ident::new(&format!("RANGE_{i}_0"), proc_macro2::Span::call_site());
            let ident_range_2 =
                syn::Ident::new(&format!("RANGE_{i}_1"), proc_macro2::Span::call_site());
            let ident = syn::Ident::new(&format!("V_DIM_{i}"), proc_macro2::Span::call_site());
            quote! {
                const #ident_range_1: usize,
                const #ident_range_2: usize,
            }
        })
        .collect::<Vec<_>>();
    let assign_ranges_idents = (0..dim_val)
        .map(|i| syn::Ident::new(&format!("V_DIM_{i}"), proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();
    let assign_ranges = (0..dim_val)
        .map(|i| syn::Ident::new(&format!("range_{i}"), proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();
    let assign_check = assign_ranges
        .iter()
        .zip(assign_ranges_idents.iter())
        .enumerate()
        .map(|(i, (range, dim))| {
            let first = syn::Ident::new(&format!("RANGE_{i}_0"), proc_macro2::Span::call_site());
            let second = syn::Ident::new(&format!("RANGE_{i}_1"), proc_macro2::Span::call_site());
            quote! {
                let #range = <
                    kindle_burn::dimensions::Range as
                        kindle_burn::dimensions::ConstRange<
                            0,
                            #dim,
                            #first,
                            #second,
                        >
                    >::new();
            }
        })
        .collect::<Vec<_>>();
    let slice_method = quote! {
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
            /// Returns the tensor containing the elements selected from the ranges
            pub fn slice<
                #(#const_ranges)*
            >(self) -> #name <
                Backend,
                Device,
                #(#ranges_idents),*,
                Kind,
            >{
                #(#check)*
                #name {
                    tensor: self.tensor.slice(
                        [#(#ranges),*]
                    ),
                    _device: std::marker::PhantomData,
                }
            }

            /// Returns a copy of the current tensor with the selected
            /// elements changed to the new ones at the selectet indices
            pub fn slice_assign<
                #(#assign_new_tensor_dims)*
                #(#assign_const_ranges)*
            > (
                self,
                values: #name <
                    Backend,
                    Device,
                    #(#assign_ranges_idents),*,
                    Kind,
                >
            ) -> Self
            where
                Backend: kindle_burn::tensor::backend::Backend,
                Device: kindle_burn::device::KindleDevice<Backend>,
                Kind: kindle_burn::tensor::BasicOps<Backend>,
            {
                #(#assign_check)*
                Self {
                    tensor: self.tensor.slice_assign(
                        [#(#assign_ranges),*],
                        values.tensor,
                    ),
                    _device: std::marker::PhantomData,
                }
            }
        }
    };
    quote! {
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
        }
        #(#swap_dims_static_methods)*
        #slice_method
    }
}
