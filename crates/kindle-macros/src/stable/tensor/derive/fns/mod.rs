use proc_macro2::TokenStream;
use quote::quote;

#[cfg(feature = "autodiff")]
mod autodiff;

mod all_dim;
mod any_dim;
mod base;
mod cat;
mod cov;
mod init;
mod matmul;
mod narrow;
mod one_hot;
mod repeat;
mod slice;
mod swap_dims;
mod t_bool;
mod t_float;
mod transpose;
mod var;

pub(crate) fn derive(dim_val: usize, name: &syn::Ident, dims: &[TokenStream]) -> TokenStream {
    let ty_dims = (0..dim_val)
        .map(|i| {
            let ident = syn::Ident::new(&format!("DIM_{i}"), proc_macro2::Span::call_site());
            quote! { #ident }
        })
        .collect::<Vec<_>>();

    let mut out: Vec<TokenStream> = vec![];

    #[cfg(feature = "autodiff")]
    {
        out.push(autodiff::derive_autodiff(dim_val, name, dims, &ty_dims));
    }
    out.push(base::derive_base(dim_val, name, dims, &ty_dims));
    out.push(transpose::derive_transpose(dim_val, name, dims, &ty_dims));
    out.push(init::derive_init(dim_val, name, dims, &ty_dims));
    out.push(slice::derive_slice(dim_val, name, dims, &ty_dims));
    out.push(swap_dims::derive_switch_dims(dim_val, name, dims, &ty_dims));
    out.push(repeat::derive_repeat(dim_val, name, dims, &ty_dims));
    out.push(cat::derive_cat(dim_val, name, dims, &ty_dims));
    out.push(narrow::derive_narrow(dim_val, name, dims, &ty_dims));
    out.push(t_bool::derive_bool(dim_val, name, dims, &ty_dims));
    out.push(t_float::derive_float(dim_val, name, dims, &ty_dims));
    out.push(one_hot::derive_one_hot(dim_val, name, dims, &ty_dims));
    if dim_val > 1 {
        // Backends only provide matmul for tensors with dim > 1
        out.push(matmul::derive_matmul(dim_val, name, dims, &ty_dims));
        // Covariance is only available for tensors with dim > 1
        out.push(cov::derive_cov(dim_val, name, dims, &ty_dims));
    }
    out.push(any_dim::derive_any_dim(dim_val, name, dims, &ty_dims));
    out.push(all_dim::derive_all_dim(dim_val, name, dims, &ty_dims));
    out.push(var::derive_var(dim_val, name, dims, &ty_dims));

    quote! {
        #(#out)*
    }
}

/*
empty
zeros
ones

dims
shape
reshape
transpose
swap_dims
flatten
squeeze
unsqueeze
unsqueeze_dim
slice
alice_assign
device
to_device
into_data
to_data
from_data
repeat
equal
cat
stack
iter_dim
narrow
chunk
from_bool
int
float
bool_not
inplace
exp
log
log1p
erf
recip
sqrt
cos
sin
tanh
from_floats
other(int)
zeros_like
ones_like
random_like
one_hot
matmul
var
var_bias
var_mean
var_mean_bias
random
to_full_precision
from_full_precision
detach
require_grad
is_require_grad
set_require_grad
cov
arange
arange_step
from_ints
other(float)
into_scalar
add
add_scalar
sub
sub_scalar
div
div_scalar
mul
mul_scalar
neg
full
mean
sum
mean_dim
sum_dim
equal_elem
greater
greater_equal
lower
lower_equal
greater_elem
greater_equal_elem
lower_elem
lower_equal_elem
mask_where
mask_fill
gather
scatter
select
select_assign
argmax
max
max_dim
max_dim_with_idx
argmin
min
min_dim
min_dim_with_idx
clamp
clamp_min
clamp_max
abs
triu
tril
powf
powf_scalar
powi
powiscalar
diagonal

new
into_primitive
from_primitive
* */
