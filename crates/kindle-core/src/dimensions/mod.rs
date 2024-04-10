/// Trait which allows for swapping two dimensions of a tensor.
pub trait Swap<const D1: usize, const D2: usize> {
    type Output;
    fn swap_dims(self) -> Self::Output;
}

pub trait ConstRange<const START: usize, const END: usize> {
    const VALID: ();
    fn new() -> std::ops::Range<usize>;
}

pub struct Range<const Min: usize, const MAX: usize, const START: usize, const END: usize> {
    _private: (),
}

impl<const MIN: usize, const MAX: usize, const START: usize, const END: usize> ConstRange<MIN, MAX>
    for Range<MIN, MAX, START, END>
{
    const VALID: () = assert!(MIN < MAX && START >= MIN && END <= MAX && START < END);
    fn new() -> std::ops::Range<usize> {
        _ = <Self as ConstRange<MIN, MAX>>::VALID;
        START..END
    }
}

// impl<const MIN: usize, const MAX: usize, const START: usize, const END: usize>
//     Range<MIN, MAX, START, END>
// where
//     Self: ConstRange<MIN, MAX>,
// {
//     pub fn new() -> std::ops::Range<usize> {
//         _ = <Self as ConstRange<MIN, MAX>>::VALID;
//         START..END
//     }
// }

// Flatten
// Reshape
// Squeeze
// Squeeze_dim
// Unsqueeze
// Unsqueeze_dim
