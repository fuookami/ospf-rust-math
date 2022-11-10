use crate::math::algebra::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct URtn16 {
    num: u16,
    den: u16
}

impl URtn16 {
    pub fn new(num: u16, den: u16) -> Self {
        let divisor = ordinary::gcd(num, den);
        Self {
            num: num / divisor,
            den: den / divisor
        }
    }
}

impl PartialOrd for URtn16 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = (self.num as u32) * (other.den as u32);
        let rhs = (other.num as u32) * (self.den as u32);
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for URtn16 {
    fn cmp(&self, other: &Self) ->std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for URtn16 {
    const zero: Self = Self::new(u16::zero, u16::one);
    const one: Self = Self::new(u16::one, u16::one);
}
