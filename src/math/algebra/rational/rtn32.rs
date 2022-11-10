use crate::math::algebra::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rtn32 {
    num: i32,
    den: i32,
}

impl Rtn32 {
    pub fn new(num: i32, den: i32) -> Self {
        let divisor = ordinary::gcd(num, den);
        let negative = (num < i32::zero) ^ (den < i32::zero);
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

impl PartialOrd for Rtn32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = (self.num as i64) * (other.den as i64);
        let rhs = (other.num as i64) * (self.den as i64);
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for Rtn32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for Rtn32 {
    const zero: Self = Self::new(i32::zero, i32::one);
    const one: Self = Self::new(i32::one, i32::one);
}
