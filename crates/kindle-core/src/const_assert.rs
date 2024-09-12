#[allow(path_statements)]

pub trait ConstRange<const MIN: usize, const MAX: usize, const START: usize, const DIM: usize> {
    const VALID_RANGE: () = assert!(MIN < MAX, "Invalid range: MIN > MAX");
    const VALID_START: () = assert!(START >= MIN, "Invalid start provided: START < MIN",);
    const VALID_LEN: () = assert!(
        START + DIM <= MAX,
        "Invalid range provided: START + DIM > MAX"
    );
    const VALID_DIM: () = assert!(DIM != 0, "Invalid DIM provided");
    fn new() -> std::ops::Range<usize> {
        Self::VALID_RANGE;
        Self::VALID_START;
        Self::VALID_LEN;
        Self::VALID_DIM;
        START..START + DIM
    }
    fn check() {
        Self::VALID_RANGE;
        Self::VALID_START;
        Self::VALID_LEN;
        Self::VALID_DIM;
    }
}

pub trait ConstValueBetween<const VALUE: usize, const MIN: usize, const MAX: usize> {
    const VALID: () = assert!(
        MIN <= VALUE && VALUE < MAX,
        "VALUE is not between MIN and MAX",
    );
}

pub struct Range(());
pub struct Value(());

impl<const MIN: usize, const MAX: usize, const START: usize, const DIM: usize>
    ConstRange<MIN, MAX, START, DIM> for Range
{
}

impl<const VALUE: usize, const MIN: usize, const MAX: usize> ConstValueBetween<VALUE, MIN, MAX>
    for Value
{
}
