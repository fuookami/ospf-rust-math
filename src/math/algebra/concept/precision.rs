use super::Arithmetic;

pub trait Precision: Arithmetic {
    fn decimal_digits() -> Option<usize> {
        None
    }

    fn decimal_precision() -> Self {
        Self::zero()
    }
}

impl Precision for f32 {
    fn decimal_digits() -> Option<usize> {
        Some(<f32>::DIGITS)
    }

    fn decimal_precision() -> Self {
        <f32>::EPSILON
    }
}

impl Precision for f64 {
    fn decimal_digits() -> Option<usize> {
        Some(<f64>::DIGITS)
    }

    fn decimal_precision() -> Self {
        <f64>::EPSILON
    }
}
