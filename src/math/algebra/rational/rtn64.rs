use crate::math::algebra::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rtn64 {
    num: i64,
    den: i64,
}

impl Rtn64 {
    pub fn new(num: i64, den: i64) -> Self {
        let divisor = ordinary::gcd(num, den);
        let negative = (num < i64::zero) ^ (den < i64::zero);
        if negative {
            Self {
                num: num.abs().neg() / divisor,
                den: den.abs() / divisor
            }
        } else {
            Self {
                num: num.abs() / divisor,
                den: den.abs() / divisor
            }
        }
    }
}

impl PartialOrd for Rtn64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = (self.num as i128) * (other.den as i128);
        let rhs = (other.num as i128) * (self.den as i128);
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for Rtn64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for Rtn64 {
    const zero: Self = Self::new(i64::zero, i64::one);
    const one: Self = Self::new(i64::one, i64::one);
}
