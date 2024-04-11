pub trait ConstRange<const MIN: usize, const MAX: usize, const START: usize, const DIM: usize> {
    const VALID: () = assert!(MIN < MAX && START >= MIN && START + DIM <= MAX && DIM != 0);
    fn new() -> std::ops::Range<usize> {
        _ = Self::VALID;
        START..START + DIM
    }
}

pub trait ConstValue<const VALUE: usize, const EQ: usize> {
    const VALID: () = assert!(VALUE == EQ);
    fn validate() -> usize {
        _ = Self::VALID;
        VALUE
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

impl<const VALUE: usize, const EQ: usize> ConstValue<VALUE, EQ> for Value {}
