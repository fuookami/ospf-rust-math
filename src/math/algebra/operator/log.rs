use crate::math::ordinary;
use rust_decimal::Decimal;

pub trait Log<Base = Self> {
    type Output;

    fn log(self, base: Base) -> Option<Self::Output>;
    fn lg2(self) -> Option<Self::Output>;
    fn lg(self) -> Option<Self::Output>;
    fn ln(self) -> Option<Self::Output>;
}

fn log<Lhs: Log<Rhs>, Rhs>(lhs: Lhs, rhs: Rhs) -> Option<Lhs::Output> {
    lhs.log(rhs)
}

fn lg2<Lhs: Log<Rhs>, Rhs>(lhs: Lhs) -> Option<Lhs::Output> {
    lhs.lg2()
}

fn lg<Lhs: Log<Rhs>, Rhs>(lhs: Lhs) -> Option<Lhs::Output> {
    lhs.lg()
}

fn ln<Lhs: Log<Rhs>, Rhs>(lhs: Lhs) -> Option<Lhs::Output> {
    lhs.ln()
}

macro_rules! int_log_template {
    ($($type:ty)*) => ($(
        impl Log<f64> for $type {
            type Output = f64;

            fn log(self, base: f64) -> Option<Self::Output> {
                Some((self as f64).log(base))
            }

            fn lg2(self) -> Option<Self::Output> {
                Some((self as f64).log2())
            }

            fn lg(self) -> Option<Self::Output> {
                Some((self as f64).log10())
            }

            fn ln(self) -> Option<Self::Output> {
                Some((self as f64).ln())
            }
        }
    )*)
}
int_log_template! { i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 }

macro_rules! floating_log_template {
    ($($type:ty)*) => ($(
        impl Log for $type {
            type Output = Self;

            fn log(self, base: Self) -> Option<Self::Output> {
                Some(self.log(base))
            }

            fn lg2(self) -> Option<Self::Output> {
                Some(self.log2())
            }

            fn lg(self) -> Option<Self::Output> {
                Some(self.log10())
            }

            fn ln(self) -> Option<Self::Output> {
                Some(self.ln())
            }
        }
    )*);
}
floating_log_template! { f32 f64 }

impl Log for Decimal {
    type Output = Self;

    fn log(self, base: Self) -> Option<Self::Output> {
        ordinary::log(base, self)
    }

    fn lg2(self) -> Option<Self::Output> {
        ordinary::lg2(self)
    }

    fn lg(self) -> Option<Self::Output> {
        ordinary::lg10(self)
    }

    fn ln(self) -> Option<Self::Output> {
        ordinary::ln(self)
    }
}
