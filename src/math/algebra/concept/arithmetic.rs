use crate::math::algebra::operator::Reciprocal;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

pub trait Arithmetic: Sized + Clone + PartialOrd + PartialEq {
    fn zero() -> Self;
    fn one() -> Self;

    fn equiv<Rhs: Arithmetic>(&self, rhs: &Rhs) -> bool;
}

pub fn equiv<Lhs: Arithmetic, Rhs: Arithmetic>(lhs: &Lhs, rhs: &Rhs) -> bool {
    lhs.equiv(rhs)
}

pub trait PlusSemiGroup: Arithmetic + Add<Output = Self> + AddAssign {}
pub trait PlusGroup: PlusSemiGroup + Neg<Output = Self> + Sub<Output = Self> + SubAssign {}

pub trait TimesSemiGroup: Arithmetic + Mul<Output = Self> + MulAssign {}
pub trait TimesGroup:
    Arithmetic + Reciprocal<Output = Self> + Div<Output = Self> + DivAssign + Rem<Output = Self>
{
}

pub trait NumberRing: PlusGroup + TimesSemiGroup {}
pub trait NumberField: NumberRing + TimesGroup {}
