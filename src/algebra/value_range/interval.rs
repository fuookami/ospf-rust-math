use super::interval_stc::*;
use crate::functional::predicate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    Open,
    Closed,
}

impl Interval {
    pub fn to_lb_sign(&self) -> str {
        match self {
            Self::Open => Open::to_lb_sign(),
            Self::Closed => Closed::to_lb_sign(),
        }
    }

    pub fn to_ub_sign(&self) -> str {
        match self {
            Self::Open => Open::to_ub_sign(),
            Self::Closed => Closed::to_ub_sign(),
        }
    }

    pub fn union(self, rhs: Self) -> Self {
        if self == Self::Closed || rhs == Self::Closed {
            Self::Closed
        } else {
            Self::Open
        }
    }

    pub fn intersect(self, rhs: Self) -> Self {
        if self == Self::Open || rhs == Self::Open {
            Self::Open
        } else {
            Self::Closed
        }
    }

    pub fn lb_op<T>(self) -> Box<Comparator<T>> {
        match self {
            Self::Open => Open::lb_op::<T>(),
            Self::Closed => Closed::lb_op::<T>(),
        }
    }

    pub fn lb_op_with<T>(self, precision: T) -> Box<Comparator<T>> {
        match self {
            Self::Open => Open::lb_op_with::<T>(precision),
            Self::Closed => Closed::lb_op_with::<T>(precision),
        }
    }

    pub fn ub_op<T>(self) -> Box<Comparator<T>> {
        match self {
            Self::Open => Open::ub_op::<T>(),
            Self::Closed => Closed::ub_op::<T>(),
        }
    }

    pub fn ub_op_with<T>(self, precision: T) -> Box<Comparator<T>> {
        match self {
            Self::Open => Open::ub_op_with::<T>(precision),
            Self::Closed => Closed::ub_op_with::<T>(precision),
        }
    }
}
