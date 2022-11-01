use super::*;
use meta_programming::MetaJudgement;

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

pub struct IsVariant<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> MetaJudgement for IsVariant<T> {
    default const VALUE: bool = false;
}

impl<T: Variant> MetaJudgement for IsVariant<T> {
    const VALUE: bool = true;
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
invariant_template! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 }
