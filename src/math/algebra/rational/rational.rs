use crate::math::algebra::*;

pub trait Rational<I: Integer + NumberField>: RationalNumber<I> {
    fn num(&self) -> I;
    fn den(&self) -> I;
}
