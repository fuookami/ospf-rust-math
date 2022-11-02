use crate::math::algebra::concept::{RealNumber, TimesGroup, TimesSemiGroup};

pub(self) fn powPosImpl<T: TimesSemiGroup>(value: T, base: &T, index: i64) -> T {
    if index == 0 {
        T::one()
    } else {
        powPosImpl(value * base.clone(), base, index - 1)
    }
}

pub(self) fn powNegImpl<T: TimesGroup>(value: T, base: &T, index: i64) -> T {
    if index == 0 {
        T::one()
    } else {
        powNegImpl(value / base.clone(), base, index + 1)
    }
}

#[derive(Debug)]
struct NegativeIndexError<T: std::fmt::Debug> {
    index: i64,
    _marker: std::marker::PhantomData<T>,
}

impl<T: std::fmt::Debug> NegativeIndexError<T> {
    fn new(index: i64) -> Self {
        Self {
            index: index,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for NegativeIndexError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid argument for negative index \'{}\' exponential function: {}",
            self.index,
            core::any::type_name::<T>()
        )
    }
}

pub(crate) fn pow_times_semi_group<T: RealNumber + TimesSemiGroup + std::fmt::Debug>(
    base: &T,
    index: i64,
) -> Result<T, NegativeIndexError<T>> {
    if index >= 1 {
        Ok(powPosImpl(T::one().clone(), base, index))
    } else if index <= -1 {
        Err(NegativeIndexError::new(index))
    } else {
        Ok(T::one().clone())
    }
}

pub(crate) fn pow_times_group<T: RealNumber + TimesGroup + std::fmt::Debug>(
    base: &T,
    index: i64,
) -> T {
    if index >= 1 {
        powPosImpl(T::one().clone(), base, index)
    } else if index <= -1 {
        powNegImpl(T::one().clone(), base, index)
    } else {
        T::one().clone()
    }
}
