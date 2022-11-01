pub trait Pow {
    type Output;

    fn pow(index: i64) -> Self::Output;
    fn square() -> Self::Output;
    fn cubic() -> Self::Output;
}

pub trait PowF<Index = Self> {
    type Output;

    fn powf(index: Index) -> Self::Output;

    fn sqr() -> Self::Output;
    fn cbr() -> Self::Output;
}

pub trait Exp {
    type Output;

    fn exp(&self) -> Self::Output;
}
