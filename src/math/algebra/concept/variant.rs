use super::*;
use rust_decimal::Decimal;

pub trait Variant: Arithmetic {
    type ValueType: Arithmetic;

    fn value(&self) -> Option<&Self::ValueType> {
        None
    }
}

pub trait Invariant: Arithmetic {
    type ValueType: Arithmetic;

    fn value(&self) -> &Self::ValueType;
}

macro_rules! invariant_template {
    ($($type:ty)*) => ($(
        impl Invariant for $type {
            type ValueType = Self;

            fn value(&self) -> &Self::ValueType {
                &self
            }
        }
    )*)
}
invariant_template! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 Decimal }
