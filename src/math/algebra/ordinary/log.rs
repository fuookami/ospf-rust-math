use crate::math::algebra::concept::{NumberField, RealNumber};
use crate::math::algebra::operator::Pow;

pub fn log<T: RealNumber + NumberField + Pow>(base: &T, x: &T) -> T {
    return ln(x) / ln(base);
}

pub fn ln<T: RealNumber + NumberField + Pow>(x: &T) -> T {
    let mut val = T::zero();
    let mut base = (x.clone() - T::one()) / (x.clone() + T::one());
    let mut i = T::one();
    let mut j = base;
    loop {
        let thisItem = T::one() / (T::two() * i + T::one()) * j;
        val += T::two() * thisItem;
        i += T::one();
        j *= base * base;

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
