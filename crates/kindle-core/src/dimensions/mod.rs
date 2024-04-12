/// Trait which allows for swapping two dimensions of a tensor.
pub trait Swap<const D1: usize, const D2: usize> {
    type Output;
    fn swap_dims(self) -> Self::Output;
}

/// Trait which allows for repeating an element of the tensor along a dimension N times
pub trait Repeat<const DIM: usize, const TIMES: usize> {
    type Output;
    fn repeat(self) -> Self::Output;
}

// Trait which allows for concatenating tensors along a dimension.
// pub trait Cat<const DIM: usize, const N: usize, const SUM: usize> {
//     const VALID: ();
//     type Output;
//     fn cat(tensors: &[Self; N]) -> Self::Output
//     where
//         Self: Sized;
// }

// Flatten
// Reshape
// Squeeze
// Squeeze_dim
// Unsqueeze
// Unsqueeze_dim
// Stack
