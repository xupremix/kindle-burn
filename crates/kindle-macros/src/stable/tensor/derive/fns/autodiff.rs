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
            Device: kindle_burn::device::KindleDevice<'dv>,
        {
            /// Perform the backward pass of the tensor.
            fn backward(&self) -> <Backend as kindle_burn::tensor::backend::AutodiffBackend>::Gradients {
                self.tensor.backward()
            }
            // Get the gradient of the tensor.
            fn grad(
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
            fn grad_remove(
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
            fn grad_replace(
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
    }
}
