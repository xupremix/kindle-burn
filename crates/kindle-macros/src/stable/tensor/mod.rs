use proc_macro::TokenStream;
use quote::quote;

mod derive;
mod parse_helper;

pub(crate) fn define_tensor(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as parse_helper::TensorDefinition);
    if let Some(err) = input.check() {
        return err;
    }
    let vis = input.vis();
    let dim = input.dim();
    let dim_val = match dim.base10_parse::<usize>() {
        Ok(dim) => dim,
        Err(_) => {
            return syn::Error::new(dim.span(), "Dimension value must be a valid integer")
                .to_compile_error()
                .into();
        }
    };
    let name = syn::Ident::new(&format!("Tensor{dim}"), proc_macro2::Span::call_site());
    let dims = (0..dim_val)
        .map(|i| {
            let ident = syn::Ident::new(&format!("DIM_{i}"), proc_macro2::Span::call_site());
            quote! {const #ident: usize}
        })
        .collect::<Vec<_>>();
    let impl_generics = quote! {
        Backend,
        Device,
        #(#dims),*,
        Kind = kindle_burn::tensor::Float,
    };
    let where_clause = quote! {
        Backend: kindle_burn::tensor::backend::Backend,
        Kind: kindle_burn::tensor::TensorKind<Backend>,
    };
    let derive_methods = derive::derive(dim_val, &name, &dims);
    let doc = format!(r#"# A {dim_val}-Dimensional Tensor"#);
    quote! {
        #[doc = #doc]
        #[derive(Debug)]
        #vis struct #name <
            #impl_generics
        > where
            #where_clause
        {
            tensor: kindle_burn::tensor::Tensor<
                Backend,
                #dim,
                Kind,
            >,
            _device: std::marker::PhantomData<Device>,
        }
        #derive_methods
    }
    .into()
}

pub(crate) fn tensor(input: TokenStream) -> TokenStream {
    quote! {}.into()
}
