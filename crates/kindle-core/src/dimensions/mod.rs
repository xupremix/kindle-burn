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

/// Trait which allows for narrowing a tensor along a dimension.
pub trait Narrow<const DIM: usize, const START: usize, const LENGTH: usize> {
    type Output;
    fn narrow(self) -> Self::Output;
}

/// Trait which tests for an evaluation of True along a dimension
pub trait AnyDim<const DIM: usize> {
    type Output;
    fn any_dim(self) -> Self::Output;
}

// Flatten
// Reshape
// Squeeze
// Squeeze_dim
// Unsqueeze
// Unsqueeze_dim
// Stack
