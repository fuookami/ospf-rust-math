use crate::math::algebra::{Arithmetic, Precision};
use crate::math::operator::Abs;

pub struct Equal<T: Arithmetic + Abs<Output = T>> {
    precision: T,
}

impl<T: Arithmetic + Abs<Output = T>> Equal<T> {
    fn new()
    where
        T: Precision,
    {
        Self {
            precision: Self::DECIMAL_PRECISION,
        }
    }

    fn new_with(precision: T) -> Self {
        Self {
            precision: precision,
        }
    }
}

impl<T: Arithmetic + Abs<Output = T>> FnOnce<(T, T)> for Equal<T> {
    type Output = bool;
}
