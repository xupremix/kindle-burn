/// Trait which allows for swapping two dimensions of a tensor.
pub trait Swap<const D1: usize, const D2: usize> {
    type Output;
    fn swap_dims(self) -> Self::Output;
}

pub trait ConstRange<const MIN: usize, const MAX: usize, const START: usize, const DIM: usize> {
    const START: usize = START;
    const DIM: usize = DIM;
    const VALID: ();
    fn new() -> std::ops::Range<usize>;
}

pub struct Range<const START: usize, const DIM: usize> {
    _private: (),
}

impl<const MIN: usize, const MAX: usize, const START: usize, const DIM: usize>
    ConstRange<MIN, MAX, START, DIM> for Range<START, DIM>
{
    const VALID: () = assert!(MIN < MAX && START >= MIN && START + DIM <= MAX);
    fn new() -> std::ops::Range<usize> {
        _ = <Self as ConstRange<MIN, MAX, START, DIM>>::VALID;
        START..DIM
    }
}

// Flatten
// Reshape
// Squeeze
// Squeeze_dim
// Unsqueeze
// Unsqueeze_dim
