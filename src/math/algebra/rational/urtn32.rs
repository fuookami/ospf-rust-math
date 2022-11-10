use crate::math::algebra::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct URtn32 {
    num: u32,
    den: u32
}

impl URtn32 {
    pub fn new(num: u32, den: u32) -> Self {
        let divisor = ordinary::gcd(num, den);
        Self { 
            num: num / divisor, 
            den: den / divisor
        }
    }
}

impl PartialOrd for URtn32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = (self.num as u64) * (other.den as u64);
        let rhs = (other.num as u64) * (self.den as u64);
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for URtn32 {
    fn cmp(&self, other: &Self) ->std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for URtn32 {
    const zero: Self = Self::new(u32::zero, u32::one);
    const one: Self = Self::new(u32::one, u32::one);
}
