use crate::math::algebra::*;

pub trait Bound: Sized {
    fn minimum() -> Option<Self>;
    fn maximum() -> Option<Self>;
    fn positive_minimum() -> Self;
}

macro_rules! int_bound_template {
    ($($type:ty)*) => ($(
        impl Bound for $type {
            fn minimum() -> Option<Self> {
                Some(<$type>::MIN)
            }

            fn maximum() -> Option<Self> {
                Some(<$type>::MAX)
            }

            fn positive_minimum() -> Self {
                Self::one
            }
        }
    )*)
}
int_bound_template! { i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 }

macro_rules! floating_bound_template {
    ($($type:ty)*) => ($(
        impl Bound for $type {
            fn minimum() -> Self {
                <$type>::MIN
            }

            fn maximum() -> Self {
                <$type>::MAX
            }

            fn positive_minimum() -> Self {
                Self::epsilon()
            }
        }
    )*)
}
floating_bound_template! { f32 f64 Decimal }

impl Bound for IntX {
    fn minimum() -> Option<Self> {
        None
    }

    fn maximum() -> Option<Self> {
        None
    }

    fn positive_minimum() -> Self {
        IntX::from(1)
    }
}

impl Bound for UIntX {
    fn minimum() -> Option<Self> {
        Some(UIntX::from(0))
    }

    fn maximum() -> Option<Self> {
        None
    }

    fn positive_minimum() -> Self {
        UIntX::from(1)
    }
}
