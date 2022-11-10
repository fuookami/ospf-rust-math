use crate::math::algebra::operator::{Neg, Reciprocal};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};

pub trait Arithmetic: Sized + Clone + PartialOrd + PartialEq {
    const zero: Self;
    const one: Self;
}

pub trait PlusSemiGroup: Arithmetic + Add<Output = Self> + AddAssign {}
impl<T: Arithmetic + Add<Output = Self> + AddAssign> PlusSemiGroup for T {}

pub trait PlusGroup: PlusSemiGroup + Neg<Output = Self> + Sub<Output = Self> + SubAssign {}
impl<T: PlusSemiGroup + Neg<Output = Self> + Sub<Output = Self> + SubAssign> PlusGroup for T {}

pub trait TimesSemiGroup: Arithmetic + Mul<Output = Self> + MulAssign {}
impl<T: Arithmetic + Mul<Output = Self> + MulAssign> TimesSemiGroup for T {}

pub trait TimesGroup:
    Arithmetic + Reciprocal<Output = Self> + Div<Output = Self> + DivAssign + Rem<Output = Self>
{
}
impl<
        T: Arithmetic
            + Reciprocal<Output = Self>
            + Div<Output = Self>
            + DivAssign
            + Rem<Output = Self>,
    > TimesGroup for T
{
}

pub trait NumberRing: PlusGroup + TimesSemiGroup {}
impl<T: PlusGroup + TimesSemiGroup> NumberRing for T {}

pub trait NumberField: NumberRing + TimesGroup {}
impl<T: NumberRing + TimesGroup> NumberField for T {}

struct Infinity {}
const INF: Infinity = Infinity {};

struct NegativeInfinity {}
const NEG_INF: NegativeInfinity = NegativeInfinity {};
