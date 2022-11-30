use crate::math::algebra::*;

enum ValueWrapper<T: Arithmetic> {
    Value(T),
    NegInf,
    Inf
}

impl<T: Arithmetic> From<Infinity> for ValueWrapper<T> {
    fn from(_: Infinity) -> Self {
        Self::Inf
    }
}

impl<T: Arithmetic> From<NegativeInfinity> for ValueWrapper<T> {
    fn from(_: NegativeInfinity) -> Self {
        Self::NegInf
    }
}

impl<T: Arithmetic> From<T> for ValueWrapper<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}
