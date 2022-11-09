use crate::math::algebra::concept::FloatingNumber;
use std::ops::Neg;

pub fn log<T: FloatingNumber + Neg<Output = T>>(nature: T, x: T) -> Option<T> {
    if let (Some(ln_nature), Some(ln_x)) = (ln(nature), ln(x)) {
        Some(ln_x / ln_nature)
    } else {
        None
    }
}

pub fn ln<T: FloatingNumber + Neg<Output = T>>(x: T) -> Option<T> {
    if x <= T::zero() {
        T::nan()
    } else {
        let frac_e = T::e().reciprocal();

        let mut val = T::zero();
        let mut xp = x;
        if xp < T::one() {
            while xp <= frac_e {
                xp *= T::e();
                val -= T::one();
            }
        } else if xp > T::one() {
            while xp >= T::e() {
                xp /= T::e();
                val += T::one();
            }
        }
        let mut base = xp.clone() - T::one();
        let mut signed = T::one();
        let mut i = T::one();
        loop {
            let this_item = signed.clone() * base.clone() / i.clone();
            val += this_item.clone();
            base *= xp.clone() - T::one();
            signed = -signed;
            i += T::one();

            if this_item <= T::epsilon() {
                break;
            }
        }
        Some(val)
    }
}

pub fn lg10<T: FloatingNumber + Neg<Output = T>>(x: T) -> Option<T> {
    return log(T::ten(), x);
}

pub fn lg2<T: FloatingNumber + Neg<Output = T>>(x: T) -> Option<T> {
    return log(T::two(), x);
}
