use super::*;
use crate::math::algebra::{ IntX, UIntX };
use crate::math::algebra::operator::{
    Abs, Cross, Exp, IntDiv, Log, Pow, PowF, RangeTo, Reciprocal,
};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use std::ops::{Div, Mul, Neg, Rem};

pub trait Scalar: Arithmetic + PlusSemiGroup + TimesSemiGroup + Bound + Cross + Abs {}

pub trait RealNumber: Scalar + Precision + Invariant {
    fn two() -> Self;
    fn three() -> Self;
    fn ten() -> Self;

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
        match Self::inf() {
            Some(inf_value) => *self == inf_value,
            None => false,
        }
    }

    fn is_neg_inf(&self) -> bool {
        match Self::neg_inf() {
            Some(inf_value) => *self == inf_value,
            None => false,
        }
    }
}

pub trait Integer: RealNumber + RangeTo + Log<f64> + PowF<f64> + Exp + Ord + Eq {}
pub trait IntegerNumber: Integer + NumberField + Pow {}
pub trait UIntegerNumber: Integer + NumberField + Pow {}

pub trait RationalNumber<I: Integer + NumberField>:
    RealNumber + NumberField + Log<f64> + PowF<f64> + Exp + Pow + Ord + Eq
{
    fn num(&self) -> &I;
    fn den(&self) -> &I;
}

pub trait FloatingNumber: RealNumber + NumberField + Log + PowF + Exp + Pow {
    fn pi() -> Self;
    fn e() -> Self;

    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn round(&self) -> Self;
    fn trunc(&self) -> Self;
    fn fract(&self) -> Self;
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
    + Ord
    + Eq
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
    + Ord
    + Eq
{
}

macro_rules! int_real_number_template {
    ($($type:ty)*) => ($(
        impl Arithmetic for $type {
            const zero: Self = 0;
            const one: Self = 1;
        }

        impl Scalar for $type {}

        impl RealNumber for $type {
            fn two() -> Self {
                2
            }

            fn three() -> Self {
                3
            }

            fn ten() -> Self {
                10
            }
        }

        impl Integer for $type {}
        impl IntegerNumber for $type {}
    )*)
}
int_real_number_template! { i8 i16 i32 i64 i128 }

impl Arithmetic for IntX {
    const zero: Self = IntX::from(0);
    const one: Self = IntX::from(1);
}

impl Scalar for IntX {}

impl RealNumber for IntX {
    fn two() -> Self {
        IntX::from(2)
    }

    fn three() -> Self {
        IntX::from(3)
    }

    fn ten() -> Self {
        IntX::from(10)
    }
}

impl Integer for IntX {}
impl IntegerNumber for IntX {}

macro_rules! uint_real_number_template {
    ($($type:ty)*) => ($(
        impl Arithmetic for $type {
            const zero: Self = 0;
            const one: Self = 1;
        }

        impl Scalar for $type {}

        impl RealNumber for $type {
            fn two() -> Self {
                2
            }

            fn three() -> Self {
                3
            }

            fn ten() -> Self {
                10
            }
        }

        impl Integer for $type {}
        impl UIntegerNumber for $type {}
    )*)
}
uint_real_number_template! { u8 u16 u32 u64 u128 }

impl Arithmetic for UIntX {
    const zero: Self = UIntX::from(0);
    const one: Self = UIntX::from(1);
}

impl Scalar for UIntX {}

impl RealNumber for UIntX {
    fn two() -> Self {
        UIntX::from(2u64)
    }

    fn three() -> Self {
        UIntX::from(3u64)
    }

    fn ten() -> Self {
        UIntX::from(10u64)
    }
}

impl Integer for UIntX {}
impl IntegerNumber for UIntX {}

macro_rules! floating_real_number_template {
    ($($type:ty)*) => ($(
        impl Arithmetic for $type {
            const zero: Self = 0.;
            const one: Self = 1.;
        }

        impl Scalar for $type {}

        impl RealNumber for $type {
            fn two() -> Self {
                2.
            }

            fn three() -> Self {
                3.
            }

            fn ten() -> Self {
                10.
            }

            fn nan() -> Option<Self> {
                Some(<$type>::NAN)
            }

            fn inf() -> Option<Self> {
                Some(<$type>::INFINITY)
            }

            fn neg_inf() -> Option<Self> {
                Some(<$type>::NEG_INFINITY)
            }
        }
    )*)
}
floating_real_number_template! { f32 f64 }

impl FloatingNumber for f32 {
    fn pi() -> Self {
        std::f32::consts::PI
    }

    fn e() -> Self {
        std::f32::consts::E
    }

    fn floor(&self) -> Self {
        (*self).floor()
    }

    fn ceil(&self) -> Self {
        (*self).ceil()
    }

    fn round(&self) -> Self {
        (*self).round()
    }

    fn trunc(&self) -> Self {
        (*self).trunc()
    }

    fn fract(&self) -> Self {
        (*self).fract()
    }
}

impl FloatingNumber for f64 {
    fn pi() -> Self {
        std::f64::consts::PI
    }

    fn e() -> Self {
        std::f64::consts::E
    }

    fn floor(&self) -> Self {
        (*self).floor()
    }

    fn ceil(&self) -> Self {
        (*self).ceil()
    }

    fn round(&self) -> Self {
        (*self).round()
    }

    fn trunc(&self) -> Self {
        (*self).trunc()
    }

    fn fract(&self) -> Self {
        (*self).fract()
    }
}

impl Arithmetic for Decimal {
    const zero: Self = Decimal::ZERO;
    const one: Self = Decimal::ONE;
}

impl Scalar for Decimal {}

impl RealNumber for Decimal {
    fn two() -> Self {
        Decimal::TWO
    }

    fn three() -> Self {
        Decimal::from_i128(3).unwrap()
    }

    fn ten() -> Self {
        Decimal::TEN
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
}

impl FloatingNumber for Decimal {
    fn pi() -> Self {
        Decimal::PI
    }

    fn e() -> Self {
        Decimal::E
    }

    fn floor(&self) -> Self {
        self.floor()
    }

    fn ceil(&self) -> Self {
        self.ceil()
    }

    fn round(&self) -> Self {
        self.round()
    }

    fn trunc(&self) -> Self {
        self.trunc()
    }

    fn fract(&self) -> Self {
        self.fract()
    }
}
