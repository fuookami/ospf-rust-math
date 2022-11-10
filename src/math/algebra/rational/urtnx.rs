use crate::math::algebra::*;

#[derive(Clone, PartialEq, Eq)]
pub struct URtnX {
    num: UIntX,
    den: UIntX
}

impl URtnX {
    pub fn new(num: UIntX, den: UIntX) -> Self {
        let divisor = ordinary::gcd(num, den);
        Self {
            num: num / divisor,
            den: den / divisor
        }
    }
}

impl PartialOrd for URtnX {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = self.num * other.den;
        let rhs = other.num * self.den;
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for URtnX {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for URtnX {
    const zero: Self = Self::new(UIntX::zero, UIntX::one);
    const one: Self = Self::new(UIntX::one, UIntX::one);
}
