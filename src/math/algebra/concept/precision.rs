use super::*;
use crate::math::algebra::*;

pub trait Precision: Arithmetic {
    fn epsilon() -> Self;
    fn decimal_digits() -> Option<usize>;
    fn decimal_precision() -> Self;
}

default impl<T: Arithmetic> Precision for T {
    fn epsilon() -> Self {
        Self::zero
    }

    fn decimal_digits() -> Option<usize> {
        None
    }

    fn decimal_precision() -> Self {
        Self::zero
    }
}

macro_rules! int_precision_template {
    ($($type:ty)*) => ($(
        impl Precision for $type { }
    )*)
}
int_precision_template! { i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 IntX UIntX }

impl Precision for f32 {
    fn epsilon() -> Self {
        <f32>::EPSILON
    }

    fn decimal_digits() -> Option<usize> {
        Some(<f32>::DIGITS as usize)
    }

    fn decimal_precision() -> Self {
        <f32>::EPSILON
    }
}

impl Precision for f64 {
    fn epsilon() -> Self {
        <f64>::EPSILON
    }

    fn decimal_digits() -> Option<usize> {
        Some(<f64>::DIGITS as usize)
    }

    fn decimal_precision() -> Self {
        <f64>::EPSILON
    }
}

impl Precision for Decimal {
    fn epsilon() -> Self {
        Decimal::from_f64(1e-28).unwrap()
    }

    fn decimal_digits() -> Option<usize> {
        Some(28)
    }

    fn decimal_precision() -> Self {
        Self::epsilon()
    }
}
