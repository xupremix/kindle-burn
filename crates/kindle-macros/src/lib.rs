use proc_macro::TokenStream;

#[cfg(feature = "nightly")]
mod nightly;

#[cfg(not(feature = "nightly"))]
mod stable;

#[cfg(feature = "nightly")]
use nightly as macros;

#[cfg(not(feature = "nightly"))]
use stable as macros;

#[proc_macro]
pub fn define_tensor(input: TokenStream) -> TokenStream {
    macros::tensor::define_tensor(input)
}

#[proc_macro]
pub fn tensor(input: TokenStream) -> TokenStream {
    macros::tensor::tensor(input)
}
