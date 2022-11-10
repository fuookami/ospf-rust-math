use crate::math::algebra::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct URtn8 {
    num: u8,
    den: u8
}

impl URtn8 {
    pub fn new(num: u8, den: u8) -> Self {
        let divisor = ordinary::gcd(num, den);
        Self {
            num: num / divisor,
            den: den / divisor
        }
    }
}

impl PartialOrd for URtn8 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = (self.num as u16) * (other.den as u16);
        let rhs = (other.num as u16) * (self.den as u16);
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for URtn8 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for URtn8 {
    const zero: Self = Self::new(u8::zero, u8::one);
    const one: Self = Self::new(u8::one, u8::one);
}
