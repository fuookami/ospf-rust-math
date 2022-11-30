use super::*;
use crate::math::algebra::*;

pub struct Bound<T: Arithmetic> {
    value: ValueWrapper<T>,
    interval: Interval,
}

pub struct ValueRange<T: Arithmetic> {
    lb: Option<Bound<T>>,
    ub: Option<Bound<T>>,
}

impl<T: Arithmetic> ValueRange<T> {
    pub fn new() -> Self {
        Self {
            lb: Option::Some(Bound {
                value: ValueWrapper::NegInf,
                interval: Interval::Closed,
            }),
            ub: Option::Some(Bound {
                value: ValueWrapper::Inf,
                interval: Interval::Closed,
            }),
        }
    }

    pub fn new_with(lb: T, ub: T, lb_interval: Interval, ub_interval: Interval) -> Self {
        assert!(!Self::empty(&lb, &ub, lb_interval, ub_interval));
        Self {
            lb: Option::Some(Bound {
                value: ValueWrapper::Value(lb),
                interval: lb_interval,
            }),
            ub: Option::Some(Bound {
                value: ValueWrapper::Value(ub),
                interval: ub_interval,
            }),
        }
    }

    fn empty(lb: &T, ub: &T, lb_interval: Interval, ub_interval: Interval) -> bool {
        !(lb_interval.lb_op())(lb, ub) || !(ub_interval.ub_op())(lb, ub)
    }
}
