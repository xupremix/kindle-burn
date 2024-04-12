use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_float(
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
            kindle_burn::tensor::Float,
        >
        where
            Backend: kindle_burn::tensor::backend::Backend,
            Device: kindle_burn::device::KindleDevice<Backend>,
        {
            /// Perform in implace operation on the underlying tensor
            pub fn inplace<F>(&mut self, f: F)
            where
                F: FnOnce(
                    kindle_burn::tensor::Tensor<
                        Backend,
                        #dim_val,
                        kindle_burn::tensor::Float
                    >
                ) -> kindle_burn::tensor::Tensor<
                    Backend,
                    #dim_val,
                    kindle_burn::tensor::Float
                >
            {
                self.tensor.inplace(f);
            }

            /// Perform the exp fn (e^x)
            pub fn exp(self) -> Self {
                Self {
                    tensor: self.tensor.exp(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Perform the log fn (ln(x))
            pub fn log(self) -> Self {
                Self {
                    tensor: self.tensor.log(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Perform the log op (log(x + 1))
            pub fn log1p(self) -> Self {
                Self {
                    tensor: self.tensor.log1p(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Apply the error function
            pub fn erf(self) -> Self {
                Self {
                    tensor: self.tensor.erf(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Apply the element-wise reciprocal
            pub fn recip(self) -> Self {
                Self {
                    tensor: self.tensor.recip(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Apply the element-wise sin function
            pub fn sin(self) -> Self {
                Self {
                    tensor: self.tensor.sin(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Apply the element-wise cos function
            pub fn cos(self) -> Self {
                Self {
                    tensor: self.tensor.cos(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Apply the element-wise tanh function
            pub fn tanh(self) -> Self {
                Self {
                    tensor: self.tensor.tanh(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Apply the element-wise square root function
            pub fn sqrt(self) -> Self {
                Self {
                    tensor: self.tensor.sqrt(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Creates a float tensor from the given data
            /// WARNING: panics if the dimensions provided do not match
            pub fn from_float_unckecked<D>(
                data: D
            ) -> Self
            where
                D: Into<
                    kindle_burn::tensor::Data<
                        f32,
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
                        kindle_burn::tensor::Float,
                    >::from_floats(
                        data,
                        &Device::to_device(),
                    ),
                    _device: std::marker::PhantomData,
                }
            }

            /// Convert to an int tensor
            pub fn int(self) -> #name<
                Backend,
                Device,
                #(#ty_dims),*,
                kindle_burn::tensor::Int
            > {
                #name {
                    tensor: self.tensor.int(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Returns a zeroed tensor with the same dimensions
            pub fn zeros_like(&self) -> Self {
                Self {
                    tensor: self.tensor.zeros_like(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Returns a ones tensor with the same dimensions
            pub fn ones_like(&self) -> Self {
                Self {
                    tensor: self.tensor.ones_like(),
                    _device: std::marker::PhantomData,
                }
            }

            /// Returns a tensor with the same dimensions with values
            /// sampled from the given distribution
            pub fn random_like(
                &self,
                distribution: kindle_burn::tensor::Distribution,
            ) -> Self {
                Self {
                    tensor: self.tensor.random_like(distribution),
                    _device: std::marker::PhantomData,
                }
            }

        }
    }
}
