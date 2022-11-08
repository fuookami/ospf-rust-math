use crate::math::algebra::concept::{NumberField, RealNumber};
use crate::math::algebra::operator::Pow;

pub fn log<T: RealNumber + NumberField + Pow>(base: &T, x: &T) -> T {
    return ln(x) / ln(base);
}

pub fn ln<T: RealNumber + NumberField + Pow>(x: &T) -> T {
    let mut val = T::zero();
    let mut base = (x.clone() - T::one()) / (x.clone() + T::one());
    let mut i = T::one();
    loop {
        let thisItem = T::one() / (T::two() * i + T::one()) * base;
        val += T::two() * thisItem;
        i += T::one();

        if thisItem <= T::epsilon() {
            break;
        }
    }
    val
}

pub fn lg10<T: RealNumber + NumberField + Pow>(x: &T) -> T {
    return log(&T::ten(), x);
}

pub fn lg2<T: RealNumber + NumberField + Pow>(x: &T) -> T {
    return log(&T::two(), x);
}
