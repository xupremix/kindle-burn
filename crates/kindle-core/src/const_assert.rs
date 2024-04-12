pub trait ConstRange<const MIN: usize, const MAX: usize, const START: usize, const DIM: usize> {
    const VALID: () = assert!(MIN < MAX && START >= MIN && START + DIM <= MAX && DIM != 0);
    fn new() -> std::ops::Range<usize> {
        _ = Self::VALID;
        START..START + DIM
    }
}

pub trait ConstValueBetween<const VALUE: usize, const MIN: usize, const MAX: usize> {
    const VALID: () = assert!(MIN <= VALUE && VALUE < MAX);
    fn validate() {
        _ = Self::VALID;
    }
}

pub struct Range {
    _private: (),
}
pub struct Value {
    _private: (),
}

impl<const MIN: usize, const MAX: usize, const START: usize, const DIM: usize>
    ConstRange<MIN, MAX, START, DIM> for Range
{
}

impl<const VALUE: usize, const MIN: usize, const MAX: usize> ConstValueBetween<VALUE, MIN, MAX>
    for Value
{
}
