use super::*;
use crate::math::algebra::*;

pub struct BoundStc<T: Arithmetic, I: IntervalStc = Closed> {
    value: ValueWrapper<T>,
    _marker: std::marker::PhantomData<I>,
}

pub struct ValueRangeStc<T: Arithmetic, LI: IntervalStc = Closed, UI: IntervalStc = Closed> {
    lb: Option<BoundStc<T, LI>>,
    ub: Option<BoundStc<T, UI>>,
}
