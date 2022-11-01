use super::*;
use crate::math::algebra::operator::{
    Abs, Cross, Exp, IntDiv, Log, Pow, PowF, RangeTo, Reciprocal,
};
use std::ops::{Div, Mul, Neg, Rem};

pub trait Scalar: Arithmetic + PlusSemiGroup + TimesSemiGroup + Cross + Abs {}

pub trait RealNumber: Scalar + Invariant + Ord + Eq + Log + PowF + Exp {
    fn two() -> Self;
    fn three() -> Self;
    fn ten() -> Self;

    fn minimum() -> Self;
    fn maximum() -> Self;
    fn positive_minimum() -> Self;

    fn decimal_digits() -> Option<usize> {
        None
    }
    fn decimal_precision() -> Self {
        Self::zero()
    }
    fn epsilon() -> Self {
        Self::zero()
    }

    fn nan() -> Option<Self> {
        None
    }
    fn inf() -> Option<Self> {
        None
    }
    fn neg_inf() -> Option<Self> {
        None
    }

    fn is_inf(&self) -> bool {
        self == Self::inf()
    }
    fn is_neg_inf(&self) -> bool {
        self == Self::nag_inf()
    }
}

pub trait Integer: RealNumber + RangeTo {}
pub trait IntegerNumber: Integer + NumberField + Pow {}
pub trait UIntegerNumber: Integer + NumberField + Pow {}

pub trait RationalNumber<I: Integer + NumberField>: RealNumber + NumberField + Pow {}

pub trait FloatingNumber: RealNumber + NumberField + Pow {
    fn pi() -> Self;
    fn e() -> Self;
}

pub trait NumericIntegerNumber<I: IntegerNumber>:
    Integer
    + PlusGroup
    + TimesSemiGroup
    + Reciprocal
    + Div
    + IntDiv<Output = Self>
    + Rem<Output = Self>
    + Pow
{
}

pub trait NumericUIntegerNumber<I: UIntegerNumber>:
    Integer
    + PlusSemiGroup
    + TimesSemiGroup
    + Neg
    + Mul
    + Reciprocal
    + Div
    + IntDiv<Output = Self>
    + Rem<Output = Self>
    + Pow
{
}
