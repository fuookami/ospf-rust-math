use crate::math::algebra::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rtn16 {
    num: i16,
    den: i16,
}

impl Rtn16 {
    pub fn new(num: i16, den: i16) -> Self {
        let divisor = ordinary::gcd(num, den);
        let negative = (num < i16::zero) ^ (den < i16::zero);
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

impl PartialOrd for Rtn16 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = (self.num as i32) * (other.den as i32);
        let rhs = (other.num as i32) * (self.den as i32);
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for Rtn16 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for Rtn16 {
    const zero: Self = Self::new(i16::zero, i16::one);
    const one: Self = Self::new(i16::one, i16::one);
}
