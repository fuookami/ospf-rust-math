use crate::math::algebra::*;

#[derive(Clone)]
struct Rtn8 {
    num: i8,
    den: i8,
}

impl Rtn8 {
    pub fn new(num: i8, den: i8) -> Rtn8 {
        let divisor = ordinary::gcd(num, den);
        let negative = (num < i8::zero()) ^ (den < i8::zero());
        if negative {
            Rtn8 {
                num: num.abs().neg() / divisor,
                den: den.abs() / divisor,
            }
        } else {
            Rtn8 {
                num: num.abs() / divisor,
                den: den.abs() / divisor,
            }
        }
    }
}

pub struct Rtn16 {
    num: i16,
    den: i16,
}

pub struct Rtn32 {
    num: i32,
    den: i32,
}

pub struct Rtn64 {
    num: i64,
    den: i64,
}

pub struct Rtn128 {
    num: i128,
    den: i128,
}

pub struct RtnX {
    num: IntX,
    den: IntX,
}

macro_rules! rtn_template {
    ($rtn_type:ty, $type:ty) => {
        impl Arithmetic for $rtn_type {
            fn zero() -> $rtn_type {
                <$rtn_type>::new(<$type>::zero(), <$type>::one())
            }

            fn one() -> $rtn_type {
                <$rtn_type>::new(<$type>::one(), <$type>::one())
            }
        }
    };
}
rtn_template!(Rtn8, i8);
rtn_template!(Rtn16, i16);
rtn_template!(Rtn32, i32);
rtn_template!(Rtn64, i64);
rtn_template!(Rtn128, i128);
rtn_template!(URtn8, u8);
rtn_template!(URtn16, u16);
rtn_template!(URtn32, u32);
rtn_template!(URtn64, u64);
rtn_template!(URtn128, u128);
