use crate::math::algebra::{Arithmetic, Precision};
use crate::math::operator::Abs;

pub struct Zero<T: Arithmetic + Abs<Output = T>> {
    precision: T,
}

impl<T: Arithmetic + Abs<Output = T>> Zero<T> {
    fn new()
    where
        T: Precision,
    {
        Self {
            precision: Self::DECIMAL_PRECISION,
        }
    }

    fn new_with(precision: T) -> Self {
        Self {
            precision: precision,
        }
    }
}

impl<T: Arithmetic + Abs<Output = T>> FnOnce<&T> for Zero<T> {
    type Output = bool;

    extern "rust-call" fn call_once(self, args: &T) -> Self::Output {
        return args.abs() <= self.precision;
    }
}

impl<T: Arithmetic + Abs<Output = T>> FnMut<&T> for Zero<T> {
    extern "rust-call" fn call_mut(&mut self, args: &T) -> Self::Output {
        return self.call_once(args);
    }
}

impl<T: Arithmetic + Abs<Output = T>> Fn<&T> for Zero<T> {
    extern "rust-call" fn call(&mut self, args: &T) -> Self::Output {
        return self.call_once(args);
    }
}
