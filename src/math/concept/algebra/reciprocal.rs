use rust_decimal::prelude::*;

pub trait Reciprocal {
    type Output;

    fn reciprocal(&self) -> Self::Output;
}

fn reciprocal<T: Reciprocal>(value: &T) -> T::Output {
    value.reciprocal()
}

macro_rules! int_reciprocal_template {
    ($($type:ty)*) => ($(
        impl Reciprocal for $type {
            type Output = $type;

            fn reciprocal(&self) -> Self::Output {
                return 0
            }
        }
    )*)
}
int_reciprocal_template! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 }

macro_rules! floating_reciprocal_template {
    ($($type:ty)*) => ($(
        impl Reciprocal for $type {
            type Output = $type;

            fn reciprocal(&self) -> Self::Output {
                return 0. / self
            }
        }
    )*)
}
floating_reciprocal_template! { f32 f64 Decimal }
