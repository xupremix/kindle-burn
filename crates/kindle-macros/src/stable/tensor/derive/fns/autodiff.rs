use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive_autodiff(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    ty_dims: &[TokenStream],
) -> TokenStream {
    quote! {
        impl <
            'dv,
            Backend,
            Device,
            #(#dims),*
        > #name <
            'dv,
            Backend,
            Device,
            #(#ty_dims),*,
            kindle_burn::tensor::Float,
        > where
            Backend: kindle_burn::tensor::backend::AutodiffBackend,
            Device: kindle_burn::device::KindleDevice<'dv, Backend>,
        {
            /// Perform the backward pass of the tensor.
            pub fn backward(&self) -> <Backend as kindle_burn::tensor::backend::AutodiffBackend>::Gradients {
                self.tensor.backward()
            }
            // Get the gradient of the tensor.
            pub fn grad(
                &self,
                grads: &<Backend as kindle_burn::tensor::backend::AutodiffBackend>::Gradients,
            ) -> Option<
                #name<
                    'dv,
                    <Backend as kindle_burn::tensor::backend::AutodiffBackend>::InnerBackend,
                    Device,
                    #(#ty_dims),*,
                    kindle_burn::tensor::Float,
                >
            > {
                match self.tensor.grad(grads) {
                    Some(tensor) => Some(#name {
                        tensor,
                        _device: std::marker::PhantomData,
                    }),
                    None => None,
                }
            }
            /// Remove the gradient of the tensor.
            pub fn grad_remove(
                &self,
                grads: &mut <Backend as kindle_burn::tensor::backend::AutodiffBackend>::Gradients,
            ) -> Option<
                #name<
                    'dv,
                    <Backend as kindle_burn::tensor::backend::AutodiffBackend>::InnerBackend,
                    Device,
                    #(#ty_dims),*,
                    kindle_burn::tensor::Float,
                >
            > {
                match self.tensor.grad_remove(grads) {
                    Some(tensor) => Some(#name {
                        tensor,
                        _device: std::marker::PhantomData,
                    }),
                    None => None,
                }
            }
            /// Replace the gradients
            pub fn grad_replace(
                &self,
                grads: &mut <Backend as kindle_burn::tensor::backend::AutodiffBackend>::Gradients,
                grad: #name <
                    'dv,
                    <Backend as kindle_burn::tensor::backend::AutodiffBackend>::InnerBackend,
                    Device,
                    #(#ty_dims),*,
                    kindle_burn::tensor::Float,
                >
            ) {
                self.tensor.grad_replace(
                    grads,
                    grad.tensor
                )
            }
        }

        impl <
            'dv,
            Backend,
            Device,
            #(#dims),*,
            Kind,
        > #name <
            'dv,
            Backend,
            Device,
            #(#ty_dims),*,
            Kind,
        > where
            Backend: kindle_burn::tensor::backend::AutodiffBackend,
            Device: kindle_burn::device::KindleDevice<'dv, Backend>,
            Kind: kindle_burn::tensor::BasicAutodiffOps<Backend>,
        {
            /// Inner tensor without the autodiff
            pub fn inner(self) -> #name <
                'dv,
                <Backend as kindle_burn::tensor::backend::AutodiffBackend>::InnerBackend,
                Device,
                #(#ty_dims),*,
                <Kind as kindle_burn::tensor::BasicAutodiffOps<Backend>>::InnerKind,
            > {
                #name {
                    tensor: self.tensor.inner(),
                    _device: std::marker::PhantomData,
                }
            }
            /// Convert a tensor to the autodiff backend
            pub fn from_inner(
                inner: #name <
                    'dv,
                    <Backend as kindle_burn::tensor::backend::AutodiffBackend>::InnerBackend,
                    Device,
                    #(#ty_dims),*,
                    <Kind as kindle_burn::tensor::BasicAutodiffOps<Backend>>::InnerKind,
                >
            ) -> Self {
                Self {
                    tensor: kindle_burn::tensor::Tensor::from_inner(inner.tensor),
                    _device: std::marker::PhantomData,
                }
            }
        }
    }
}
