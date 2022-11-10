use crate::math::algebra::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rtn128 {
    num: i128,
    den: i128,
}

impl Rtn128 {
    pub fn new(num: i128, den: i128) -> Self {
        let divisor = ordinary::gcd(num, den);
        let negative = (num < i128::zero) ^ (den < i128::zero);
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

impl PartialOrd for Rtn128 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = IntX::from(self.num) * IntX::from(other.den);
        let rhs = IntX::from(other.num) * IntX::from(self.den);
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for Rtn128 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for Rtn128 {
    const zero: Self = Self::new(i128::zero, i128::one);
    const one: Self = Self::new(i128::one, i128::one);
}
