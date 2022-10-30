use super::Reciprocal;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

pub trait Arithmetic: Sized + Clone + PartialOrd + PartialEq {
    fn zero() -> Self;
    fn one() -> Self;

    fn equiv(&self, rhs: &Self) -> bool;
}

pub fn equiv<T: Arithmetic>(lhs: &T, rhs: &T) -> bool {
    lhs.equiv(rhs)
}

pub trait PlusSemiGroup: Arithmetic + Add<Output = Self> + AddAssign {}
pub trait PlusGroup: PlusSemiGroup + Neg<Output = Self> + Sub<Output = Self> + SubAssign {}

pub trait TimesSemiGroup: Arithmetic + Mul<Output = Self> + MulAssign {}
pub trait TimesGroup:
    Arithmetic + Reciprocal<Output = Self> + Div<Output = Self> + DivAssign + Rem<Output = Self>
{
}
