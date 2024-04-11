pub use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_slice(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
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
                    kindle_burn::const_assert::Range as
                        kindle_burn::const_assert::ConstRange<
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
    let assign_ranges_idents = (0..dim_val)
        .map(|i| syn::Ident::new(&format!("V_DIM_{i}"), proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();
    let assign_ranges = (0..dim_val)
        .map(|i| syn::Ident::new(&format!("range_{i}"), proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();
    let assign_check = assign_ranges
        .iter()
        .zip(assign_ranges_idents.iter())
        .map(|(range, dim)| {
            quote! {
                let #range = <
                    kindle_burn::const_assert::Range as
                        kindle_burn::const_assert::ConstRange<
                            0,
                            #dim,
                            0,
                            #dim,
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
        #slice_method
    }
}
