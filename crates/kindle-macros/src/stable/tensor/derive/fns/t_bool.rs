use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_bool(
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
        >   #name <
            Backend,
            Device,
            #(#ty_dims),*,
            kindle_burn::tensor::Bool,
        >
        where
            Backend: kindle_burn::tensor::backend::Backend,
            Device: kindle_burn::device::KindleDevice<Backend>,
        {
            /// Creates a boolean tensor from the given data
            /// WARNING: panics if the dimensions provided do not match
            pub fn from_bool_unchecked<Data>(
                data: Data,
            ) -> Self
            where
                Data: Into<kindle_burn::tensor::Data<
                    bool,
                    #dim_val
                >
            > {
                let curr_dims = [#(#ty_dims),*];
                let data = data.into();
                for (dim, curr_dim) in data.shape.dims.iter().zip(curr_dims.iter()) {
                    assert_eq!(dim, curr_dim, "Expected dimension {} but got {}", curr_dim, dim);
                }
                Self {
                    tensor: kindle_burn::tensor::Tensor::<
                        Backend,
                        #dim_val,
                        kindle_burn::tensor::Bool,
                    >::from_bool(
                        data,
                        &Device::to_device(),
                    ),
                    _device: std::marker::PhantomData,
                }
            }
            /// Convert to a int tensor
            pub fn int(self) -> #name<
                Backend,
                Device,
                #(#ty_dims),*,
                kindle_burn::tensor::Int,
            >{
                #name {
                    tensor: self.tensor.int(),
                    _device: std::marker::PhantomData,
                }
            }
            /// Convert to a float tensor
            pub fn float(self) -> #name<
                Backend,
                Device,
                #(#ty_dims),*,
                kindle_burn::tensor::Float,
            >{
                #name {
                    tensor: self.tensor.float(),
                    _device: std::marker::PhantomData,
                }
            }
            /// Invert the boolean tensor
            pub fn bool_not(self) -> Self {
                Self {
                    tensor: self.tensor.bool_not(),
                    _device: std::marker::PhantomData,
                }
            }
        }
    }
}
