use crate::math::algebra::*;

#[derive(Clone, PartialEq, Eq)]
pub struct RtnX {
    num: IntX,
    den: IntX,
}

impl RtnX {
    pub fn new(num: &IntX, den: &IntX) -> Self {
        let divisor = ordinary::gcd(num.clone(), den.clone());
        let negative = (num < &IntX::zero) ^ (den < &IntX::zero);
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

impl PartialOrd for RtnX {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = self.num * other.den;
        let rhs = other.num * self.den;
        Some(lhs.cmp(&rhs))
    }
}

impl Ord for RtnX {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Arithmetic for RtnX {
    const zero: Self = Self::new(&IntX::zero, &IntX::one);
    const one: Self = Self::new(&IntX::one, &IntX::one);
}
