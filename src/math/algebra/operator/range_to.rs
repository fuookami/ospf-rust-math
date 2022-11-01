pub use std::cmp::Ord;
pub use std::ops::RangeInclusive;

pub trait RangeTo
where
    Self: Ord + Sized + Clone,
{
    fn until(&self, rhs: &Self) -> RangeInclusive<Self> {
        RangeInclusive::new(self.clone(), rhs.clone())
    }
}
