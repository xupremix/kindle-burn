/// Trait which allows for swapping two dimensions of a tensor.
pub trait Swap<const D1: usize, const D2: usize> {
    type Output;
    fn swap_dims(self) -> Self::Output;
}

pub trait ConstRange<const MIN: usize, const MAX: usize, const START: usize, const DIM: usize> {
    const VALID: () = assert!(MIN < MAX && START >= MIN && START + DIM <= MAX && DIM != 0);
    fn new() -> std::ops::Range<usize> {
        _ = <Self as ConstRange<MIN, MAX, START, DIM>>::VALID;
        START..START + DIM
    }
}

pub struct Range {
    _private: (),
}

impl<const MIN: usize, const MAX: usize, const START: usize, const DIM: usize>
    ConstRange<MIN, MAX, START, DIM> for Range
{
}

// Flatten
// Reshape
// Squeeze
// Squeeze_dim
// Unsqueeze
// Unsqueeze_dim
