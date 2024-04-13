use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_base(
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
            /// Returns the shape of the tensor.
            pub fn shape(&self) -> kindle_burn::tensor::Shape<#dim_val> {
                self.tensor.shape()
            }

            /// Returns the dimensions of the tensor.
            pub fn dims(&self) -> [usize; #dim_val] {
                self.tensor.dims()
            }

            /// Returns the device of the tensor.
            pub fn device(&self) -> <Backend as kindle_burn::tensor::backend::Backend>::Device {
                self.tensor.device()
            }

            /// Returns a new tensor on the given device.
            pub fn to_device<
                NewDevice: kindle_burn::device::KindleDevice<Backend>,
            >(self) -> #name<Backend, NewDevice, #(#ty_dims),*, Kind> {
                #name {
                    tensor: self.tensor.to_device(
                        &NewDevice::to_device(),
                    ),
                    _device: std::marker::PhantomData,
                }
            }

            /// Returns the data of the tensor
            pub fn into_data(self) -> kindle_burn::tensor::Data<
                <Kind as kindle_burn::tensor::BasicOps<Backend>>::Elem,
                #dim_val
            > {
                self.tensor.into_data()
            }

            /// Returns the data of the tensor without taking ownership.
            pub fn to_data(&self) -> kindle_burn::tensor::Data<
                <Kind as kindle_burn::tensor::BasicOps<Backend>>::Elem,
                #dim_val
            > {
                self.tensor.to_data()
            }

            /// Creates a new tensor from the given data.
            ///
            /// WARNING: This function panics if the data doesn't match the dimensions provided.
            pub fn from_data_unchecked<Data>(data: Data)-> Self
            where
                Data: Into<kindle_burn::tensor::Data<
                    <Kind as kindle_burn::tensor::BasicOps<Backend>>::Elem,
                    #dim_val
                >
            > {
                let curr_dims = [#(#ty_dims),*];
                let data = data.into();
                for (dim, curr_dim) in data.shape.dims.iter().zip(curr_dims.iter()) {
                    assert_eq!(dim, curr_dim, "Expected dimension {} but got {}", curr_dim, dim);
                }
                Self {
                    tensor: kindle_burn::tensor::Tensor::from_data(
                        data,
                        &Device::to_device(),
                    ),
                    _device: std::marker::PhantomData,
                }
            }

            /// Applies element-wise equal comparison and returns the respective boolean tensor.
            pub fn equal(self, other: Self) -> #name<
                Backend,
                Device,
                #(#ty_dims),*,
                kindle_burn::tensor::Bool
            > {
                #name {
                    tensor: self.tensor.equal(other.tensor),
                    _device: std::marker::PhantomData,
                }
            }

            /// Applies element-wise not equal comparison and returns the respective boolean tensor.
            pub fn not_equal(self, other: Self) -> #name<
                Backend,
                Device,
                #(#ty_dims),*,
                kindle_burn::tensor::Bool
            > {
                #name {
                    tensor: self.tensor.not_equal(other.tensor),
                    _device: std::marker::PhantomData,
                }
            }

            /// Returns the scalar value of the tensor.
            ///
            /// WARNING: This function panics if the tensor doesn't contain a value.
            pub fn into_scalar(self) -> <
                Kind as kindle_burn::tensor::BasicOps<Backend>
            >::Elem {
                self.tensor.into_scalar()
            }

        }
    }
}
