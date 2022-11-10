use crate::math::algebra::*;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Rtn8 {
    num: i8,
    den: i8,
}

impl Rtn8 {
    pub fn new(num: i8, den: i8) -> Self {
        let divisor = ordinary::gcd(num, den);
        let negative = (num < i8::zero) ^ (den < i8::zero);
        if negative {
            Self {
                num: num.abs().neg() / divisor,
                den: den.abs() / divisor,
            }
        } else {
            Self {
                num: num.abs() / divisor,
                den: den.abs() / divisor,
            }
        }
    }
}

impl PartialOrd for Rtn8 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = (self.num as i16) * (other.den as i16);
        let rhs = (other.num as i16) * (self.den as i16);
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for Rtn8 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for Rtn8 {
    const zero: Self = Self::new(i8::zero, i8::one);
    const one: Self = Self::new(i8::one, i8::one);
}
