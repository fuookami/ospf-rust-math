use crate::algebra::*;
use std::ops::Add;

pub enum ValueWrapper<T: Arithmetic> {
    Value(T),
    NegInf,
    Inf,
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

default impl<T: Arithmetic> From<T> for ValueWrapper<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl<T: RealNumber> From<T> for ValueWrapper<T> {
    fn from(value: T) -> Self {
        if value.is_inf() {
            Self::Inf
        } else if value.is_neg_inf() {
            Self::NegInf
        } else if value.is_nan() {
            panic!("Illegal argument NaN for value range!!!")
        } else {
            Self::Value(value)
        }
    }
}

pub struct ValueWrapperCalculationArgumentError {
    msg: String,
}

impl<'a, T: Arithmetic> Add<&'a T> for &'a ValueWrapper<T>
where
    &'a T: Add<&'a T, Output = T>,
{
    type Output = Result<ValueWrapper<T>, ValueWrapperCalculationArgumentError>;

    fn add(self, rhs: &'a T) -> Self::Output {
        match self {
            ValueWrapper::Value(lhs_value) => Ok(ValueWrapper::from(lhs_value + rhs)),
            ValueWrapper::NegInf => Ok(ValueWrapper::NegInf),
            ValueWrapper::Inf => Ok(ValueWrapper::Inf),
        }
    }
}

impl<'a, T: Arithmetic> Add for &'a ValueWrapper<T>
where
    &'a T: Add<&'a T, Output = T>,
{
    type Output = Result<ValueWrapper<T>, ValueWrapperCalculationArgumentError>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            ValueWrapper::Value(lhs_value) => match rhs {
                ValueWrapper::Value(rhs_value) => Ok(ValueWrapper::from(lhs_value + rhs_value)),
                ValueWrapper::NegInf => Ok(ValueWrapper::NegInf),
                ValueWrapper::Inf => Ok(ValueWrapper::Inf),
            },
            ValueWrapper::NegInf => match rhs {
                ValueWrapper::Value(rhs_value) => Ok(ValueWrapper::NegInf),
                ValueWrapper::NegInf => Ok(ValueWrapper::NegInf),
                ValueWrapper::Inf => Err(ValueWrapperCalculationArgumentError {
                    msg: String::from("Invalid plus between inf and -inf!!!"),
                }),
            },
            ValueWrapper::Inf => match rhs {
                ValueWrapper::Value(rhs_value) => Ok(ValueWrapper::Inf),
                ValueWrapper::NegInf => Err(ValueWrapperCalculationArgumentError {
                    msg: String::from("Invalid plus between inf and -inf!!!"),
                }),
                ValueWrapper::Inf => Ok(ValueWrapper::NegInf),
            },
        }
    }
}

impl<'a, T: RealNumber> Add<&'a T> for ValueWrapper<T>
where
    &'a T: Add<&'a T, Output = T>,
{
    type Output = Result<ValueWrapper<T>, ValueWrapperCalculationArgumentError>;

    fn add(self, rhs: &'a T) -> Self::Output {
        if rhs.is_nan() {
            return Err(ValueWrapperCalculationArgumentError {
                msg: String::from("Illegal argument NaN for value range!!!"),
            });
        }

        if rhs.is_inf() {
            match self {
                ValueWrapper::Value(_) | ValueWrapper::Inf => Ok(ValueWrapper::Inf),
                ValueWrapper::NegInf => Err(ValueWrapperCalculationArgumentError {
                    msg: String::from("Invalid plus between inf and -inf!!!"),
                }),
            }
        } else if rhs.is_neg_inf() {
            match self {
                ValueWrapper::Value(_) | ValueWrapper::NegInf => Ok(ValueWrapper::NegInf),
                ValueWrapper::Inf => Err(ValueWrapperCalculationArgumentError {
                    msg: String::from("Invalid plus between inf and -inf!!!"),
                }),
            }
        } else {
            match self {
                ValueWrapper::Value(lhs_value) => Ok(ValueWrapper::from(lhs_value + rhs)),
                ValueWrapper::NegInf => Ok(ValueWrapper::NegInf),
                ValueWrapper::Inf => Ok(ValueWrapper::Inf),
            }
        }
    }
}

impl<'a, T: RealNumber> Add for &'a ValueWrapper<T>
where
    &'a T: Add<&'a T, Output = T>,
{
    type Output = Result<ValueWrapper<T>, ValueWrapperCalculationArgumentError>;

    fn add(self, rhs: Self) -> Self::Output {
        if let ValueWrapper::Value(rhs_value) = &rhs {
            if rhs_value.is_nan() {
                return Err(ValueWrapperCalculationArgumentError {
                    msg: String::from("Illegal argument NaN for value range!!!"),
                });
            }
        }

        match self {
            ValueWrapper::Value(lhs_value) => match rhs {
                ValueWrapper::Value(rhs_value) => Ok(ValueWrapper::from(lhs_value + rhs_value)),
                ValueWrapper::NegInf => Ok(ValueWrapper::NegInf),
                ValueWrapper::Inf => Ok(ValueWrapper::Inf),
            },
            ValueWrapper::NegInf => match rhs {
                ValueWrapper::Value(rhs_value) => Ok(ValueWrapper::NegInf),
                ValueWrapper::NegInf => Ok(ValueWrapper::NegInf),
                ValueWrapper::Inf => Err(ValueWrapperCalculationArgumentError {
                    msg: String::from("Invalid plus between inf and -inf!!!"),
                }),
            },
            ValueWrapper::Inf => match rhs {
                ValueWrapper::Value(rhs_value) => Ok(ValueWrapper::Inf),
                ValueWrapper::NegInf => Err(ValueWrapperCalculationArgumentError {
                    msg: String::from("Invalid plus between inf and -inf!!!"),
                }),
                ValueWrapper::Inf => Ok(ValueWrapper::NegInf),
            },
        }
    }
}
