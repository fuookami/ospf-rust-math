use crate::math::algebra::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct URtn128 {
    num: u128,
    den: u128
}

impl URtn128 {
    pub fn new(num: u128, den: u128) -> Self {
        let divisor = ordinary::gcd(num, den);
        Self {
            num: num / divisor,
            den: den / divisor
        }
    }
}

impl PartialOrd for URtn128 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = UIntX::from(self.num) * UIntX::from(other.den);
        let rhs = UIntX::from(other.num) * UIntX::from(self.den);
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for URtn128 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for URtn128 {
    const zero: Self = Self::new(u128::zero, u128::one);
    const one: Self = Self::new(u128::one, u128::one);
}
