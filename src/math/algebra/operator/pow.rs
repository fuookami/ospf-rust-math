use crate::math::algebra::ordinary::{pow_times_group, pow_times_semi_group};
use rust_decimal::Decimal;

pub trait Pow {
    type Output;

    fn pow(&self, index: i64) -> Self::Output;

    fn square(&self) -> Self::Output {
        self.pow(2)
    }

    fn cubic(&self) -> Self::Output {
        self.pow(3)
    }
}

pub trait PowF<Index = Self> {
    type Output;

    fn powf(&self, index: &Index) -> Self::Output;

    fn sqr(&self) -> Self::Output;
    fn cbr(&self) -> Self::Output;
}

pub trait Exp {
    type Output;

    fn exp(&self) -> Self::Output;
}

macro_rules! int_pow_template {
    ($($type:ty)*) => ($(
        impl Pow for $type {
            type Output = Self;

            fn pow(&self, index: i64) -> Self::Output {
                pow_times_semi_group(self, index).unwrap()
            }
        }

        impl PowF<f64> for $type {
            type Output = f64;

            fn powf(&self, index: &f64) -> Self::Output {
                (*self as f64).powf(*index)
            }

            fn sqr(&self) -> Self::Output {
                (*self as f64).sqrt()
            }

            fn cbr(&self) -> Self::Output {
                (*self as f64).cbrt()
            }
        }

        impl Exp for $type {
            type Output = f64;

            fn exp(&self) -> Self::Output {
                (*self as f64).exp()
            }
        }
    )*)
}
int_pow_template! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 }

macro_rules! floating_pow_template {
    ($($type:ty)*) => ($(
        impl Pow for $type {
            type Output = Self;

            fn pow(&self, index: i64) -> Self::Output {
                pow_times_group(self, index)
            }
        }

        impl PowF for $type {
            type Output = Self;

            fn powf(&self, index: &Self) -> Self::Output {
                self.powf(index)
            }

            fn sqr(&self) -> Self::Output {
                self.sqrt()
            }

            fn cbr(&self) -> Self::Output {
                self.cbrt()
            }
        }

        impl Exp for $type {
            type Output = Self;

            fn exp(&self) -> Self::Output {
                self.exp()
            }
        }
    )*)
}
floating_pow_template! { f32 f64 Decimal }
