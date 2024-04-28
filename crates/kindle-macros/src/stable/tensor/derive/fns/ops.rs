use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_ops(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    quote! {
        impl<
            Backend,
            Device,
            #(#dims),*,
            Kind,
        > #name <
            Backend,
            Device,
            #(#ty_dims),*,
            Kind,
        >
        where
            Backend: kindle_burn::tensor::backend::Backend,
            Device: kindle_burn::device::KindleDevice<Backend>,
            Kind: kindle_burn::tensor::Numeric<Backend>,
            <Kind as kindle_burn::tensor::BasicOps<Backend>>::Elem: kindle_burn::tensor::Element,
        {
            /// Adds another tensor to the current one
            pub fn add(self, other: Self) -> Self {
                Self {
                    tensor: self.tensor.add(other.tensor),
                    _device: std::marker::PhantomData,
                }
            }

            /// Adds a value which can be interpreted as a scalar to the tensor
            pub fn add_scalar<Scalar>(self, other: Scalar) -> Self
            where
                Scalar: kindle_burn::prelude::ElementConversion
            {
                Self {
                    tensor: self.tensor.add_scalar(other),
                    _device: std::marker::PhantomData,
                }
            }

            /// Subtracts another tensor to the current one
            pub fn sub(self, other: Self) -> Self {
                Self {
                    tensor: self.tensor.sub(other.tensor),
                    _device: std::marker::PhantomData,
                }
            }

            /// Subtracts a value which can be interpreted as a scalar to the tensor
            pub fn sub_scalar<Scalar>(self, other: Scalar) -> Self
            where
                Scalar: kindle_burn::prelude::ElementConversion
            {
                Self {
                    tensor: self.tensor.sub_scalar(other),
                    _device: std::marker::PhantomData,
                }
            }

            /// Divides another tensor with the current one
            pub fn div(self, other: Self) -> Self {
                Self {
                    tensor: self.tensor.div(other.tensor),
                    _device: std::marker::PhantomData,
                }
            }

            /// Divides the current tensor with a value which can be interpreted as a scalar
            pub fn div_scalar<Scalar>(self, other: Scalar) -> Self
            where
                Scalar: kindle_burn::prelude::ElementConversion
            {
                Self {
                    tensor: self.tensor.div_scalar(other),
                    _device: std::marker::PhantomData,
                }
            }

            /// Performs element-wise multiplication with another tensor
            pub fn mul(self, other: Self) -> Self {
                Self {
                    tensor: self.tensor.mul(other.tensor),
                    _device: std::marker::PhantomData,
                }
            }

            /// Performs element-wise multiplication with another value which can be
            /// interpreted as a Scalar data type
            pub fn mul_scalar<Scalar>(self, other: Scalar) -> Self
            where
                Scalar: kindle_burn::prelude::ElementConversion
            {
                Self {
                    tensor: self.tensor.mul_scalar(other),
                    _device: std::marker::PhantomData,
                }
            }

            /// Switches the sign of each element in the current tensor
            pub fn neg(self) -> Self {
                Self {
                    tensor: self.tensor.neg(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Returns the signs of each element
            pub fn sign(self) -> Self {
                Self {
                    tensor: self.tensor.sign(),
                    _device: std::marker::PhantomData,
                }
            }
        }
    }
}
