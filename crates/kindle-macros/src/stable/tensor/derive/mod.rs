use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn derive(
    dim_val: usize,
    name: &syn::Ident,
    dims: &[TokenStream],
    impl_generics: &TokenStream,
    where_clause: &TokenStream,
) -> TokenStream {
    quote! {}
}

/*
backward
grad
grad_remove
grad_replace
inner
from_inner
new
into_primitive
from_primitive
empty
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
zeros
ones
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

-- Traits

add(E: ElementConversion)
add(K: Numeric)
autodiff
bitxor
clone
clone_from
debug
deserialize
display
div(E)
div(K)
from
from for Param with record (float)
from for Param with record (int)
from for Param with record (bool)
module for tensor
mul(E)
mul(K)
neg
record(float)
record(bool)
record(int)
serialize
sub(E)
sub(K)
*/
