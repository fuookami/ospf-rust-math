use rust_decimal::Decimal;

pub trait IntDiv<Rhs = Self> {
    type Output;

    fn int_div(&self, rhs: &Rhs) -> Self::Output;
}

fn int_div<T: IntDiv>(lhs: &T, rhs: &T) -> T::Output {
    lhs.int_div(rhs)
}

macro_rules! int_div_template {
    ($($type:ty)*) => ($(
        impl InvDiv for $type {
            type Output = $type;

            fn int_div(&self, rhs: &Self) -> Self::Output {
                return self / rhs
            }
        }
    )*)
}
int_div_template! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 }

macro_rules! floating_div_template {
    ($($type:ty)*) => ($(
        impl InvDiv for $type {
            type Output = $type;

            fn int_div(&self, rhs: &Self) -> Self::Output {
                return self - self % rhs
            }
        }
    )*)
}
floating_div_template! { f32 f64 Decimal }
