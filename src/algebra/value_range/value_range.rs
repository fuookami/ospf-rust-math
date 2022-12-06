use super::*;
use crate::algebra::concept::*;
use crate::algebra::operator::*;

pub struct ValueRange<T: Arithmetic> {
    pub lb: Option<Bound<T>>,
    pub ub: Option<Bound<T>>,
}

impl<T: Arithmetic> ValueRange<T> {
    pub fn new() -> Self {
        Self {
            lb: Option::Some(Bound {
                value: ValueWrapper::NegInf,
                interval: Interval::Closed,
                side: BoundSide::Lower,
            }),
            ub: Option::Some(Bound {
                value: ValueWrapper::Inf,
                interval: Interval::Closed,
                side: BoundSide::Upper,
            }),
        }
    }

    pub fn new_with(lb: T, ub: T, lb_interval: Interval, ub_interval: Interval) -> Self {
        assert!(!Self::empty(&lb, &ub, lb_interval, ub_interval));
        Self {
            lb: Option::Some(Bound {
                value: ValueWrapper::Value(lb),
                interval: lb_interval,
                side: BoundSide::Lower,
            }),
            ub: Option::Some(Bound {
                value: ValueWrapper::Value(ub),
                interval: ub_interval,
                side: BoundSide::Upper,
            }),
        }
    }

    fn empty(lb: &T, ub: &T, lb_interval: Interval, ub_interval: Interval) -> bool {
        !(lb_interval.lb_op())(lb, ub) || !(ub_interval.ub_op())(lb, ub)
    }

    fn fixed(&self) -> bool where T: Precision {
        if let (Some(lower_bound), Some(upper_bound)) = (&self.lb, &self.ub) {
            lower_bound.interval == Interval::Closed
                && upper_bound.interval == Interval::Closed
                && if let (ValueWrapper::Value(lower_value), ValueWrapper::Value(upper_value)) =
                    (&lower_bound.value, &upper_bound.value)
                {
                    let eq_op = Equal::new();
                    eq_op(lower_value, upper_value)
                } else {
                    false
                }
        } else {
            false
        }
    }
}
