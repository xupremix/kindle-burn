use proc_macro2::TokenStream;
use quote::quote;

#[cfg(feature = "autodiff")]
mod autodiff;

mod init;

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
    out.push(init::derive_init(dim_val, name, dims, &ty_dims));

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
