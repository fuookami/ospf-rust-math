use rust_decimal::Decimal;

pub trait Log<Base = Self> {
    type Output;

    fn log(&self, base: &Base) -> Self::Output;
    fn lg2(&self) -> Self::Output;
    fn lg(&self) -> Self::Output;
    fn ln(&self) -> Self::Output;
}

fn log<Lhs: Log<Rhs>, Rhs>(lhs: &Lhs, rhs: &Rhs) -> Lhs::Output {
    lhs.log(rhs)
}

fn lg2<Lhs: Log<Rhs>, Rhs>(lhs: &Lhs) -> Lhs::Output {
    lhs.lg2()
}

fn lg<Lhs: Log<Rhs>, Rhs>(lhs: &Lhs) -> Lhs::Output {
    lhs.lg()
}

fn ln<Lhs: Log<Rhs>, Rhs>(lhs: &Lhs) -> Lhs::Output {
    lhs.ln()
}

macro_rules! int_log_template {
    ($($type:ty)*) => ($(
        impl Log<f64> for $type {
            type Output = f64;

            fn log(&self, base: &f64) -> Self::Output {
                (*self as f64).log(*base)
            }

            fn lg2(&self) -> Self::Output {
                (*self as f64).log2()
            }

            fn lg(&self) -> Self::Output {
                (*self as f64).log10()
            }

            fn ln(&self) -> Self::Output {
                (*self as f64).ln()
            }
        }
    )*)
}
int_log_template! { i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 }

macro_rules! floating_log_template {
    ($($type:ty)*) => ($(
        impl Log for $type {
            type Output = Self;

            fn log(&self, base: &Self) -> Self::Output {
                self.log(base)
            }

            fn lg2(&self) -> Self::Output {
                self.log2()
            }

            fn lg(&self) -> Self::Output {
                self.log10()
            }

            fn ln(&self) -> Self::Output {
                self.ln()
            }
        }
    )*);
}
floating_log_template! { f32 f64 Decimal }
