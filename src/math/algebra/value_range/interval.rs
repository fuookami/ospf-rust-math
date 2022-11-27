pub trait Interval: Clone + Copy + PartialEq + Eq {
    fn to_lower_bound_sign() -> str;
    fn to_upper_bound_sign() -> str;
}

pub trait Union<Rhs: Interval>: Interval {
    type Result: Interval;

    fn to_lower_bound_sign() -> str {
        Result::to_lower_bound_sign()
    }

    fn to_upper_bound_sign() -> str {
        Result::to_upper_bound_sign()
    }
}

pub trait Intersect<Rhs: Interval>: Interval {
    type Result: Interval;

    fn to_lower_bound_sign() -> str {
        Result::to_lower_bound_sign()
    }

    fn to_upper_bound_sign() -> str {
        Result::to_upper_bound_sign()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Open {}

impl Interval for Open {
    fn to_lower_bound_sign() -> str {
        "("
    }

    fn to_upper_bound_sign() -> str {
        ")"
    }
}

impl Union<Open> for Open {
    type Result = Open;
}

impl Union<Closed> for Open {
    type Result = Closed;
}

impl<T: Interval> Intersect<T> for Open {
    type Result = Open;
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Closed {}

impl Interval for Closed {
    fn to_lower_bound_sign() -> str {
        "["
    }

    fn to_upper_bound_sign() -> str {
        "]"
    }
}

impl<T: Interval> Union<T> for Closed {
    type Result = Closed;
}

impl Intersect<Open> for Closed {
    type Result = Open;
}

impl Intersect<Closed> for Closed {
    type Result = Closed;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum IntervalType {
    Open,
    Closed,
}

impl IntervalType {
    fn to_lower_bound_sign(&self) -> str {
        match self {
            Self::Open => Open::to_lower_bound_sign(),
            Self::Closed => Closed::to_lower_bound_sign(),
        }
    }

    fn to_upper_bound_sign(&self) -> str {
        match self {
            Self::Open => Open::to_upper_bound_sign(),
            Self::Closed => Closed::to_upper_bound_sign(),
        }
    }

    fn union(self, rhs: Self) -> Self {
        if self == Self::Closed || rhs == Self::Closed {
            Self::Closed
        } else {
            Self::Open
        }
    }

    fn intersect(self, rhs: Self) -> Self {
        if self == Self::Open || rhs == Self::Open {
            Self::Open
        } else {
            Self::Closed
        }
    }
}
