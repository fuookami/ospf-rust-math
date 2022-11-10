use crate::math::algebra::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct URtn64 {
    num: u64,
    den: u64
}

impl URtn64 {
    pub fn new(num: u64, den: u64) -> Self {
        let divisor = ordinary::gcd(num, den);
        Self {
            num: num / divisor,
            den: den / divisor
        }
    }
}

impl PartialOrd for URtn64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = (self.num as u128) * (other.den as u128);
        let rhs = (other.num as u128) * (self.den as u128);
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for URtn64 {
    fn cmp(&self, other: &Self) ->std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for URtn64 {
    const zero: Self = Self::new(u64::zero, u64::one);
    const one: Self = Self::new(u64::one, u64::one);
}
