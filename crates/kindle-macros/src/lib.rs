use proc_macro::TokenStream;

mod nightly;
mod stable;

use stable as macros;

#[proc_macro]
pub fn define_tensor(input: TokenStream) -> TokenStream {
    macros::tensor::define_tensor(input)
}
