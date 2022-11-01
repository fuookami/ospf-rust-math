use rust_decimal::Decimal;

pub trait Cross<Rhs = Self> {
    type Output;

    fn cross(&self, rhs: &Rhs) -> Self::Output;
}

fn cross<T: Cross>(lhs: &T, rhs: &T) -> T::Output {
    lhs.cross(rhs)
}

macro_rules! cross_template {
    ($($type:ty)*) => ($(
        impl Cross for $type {
            type Output = $type;

            fn cross(&self, rhs: &Self) -> Self::Output {
                return self * rhs
            }
        }
    )*)
}
cross_template! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 Decimal }
