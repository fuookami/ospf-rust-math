pub trait Log<Base = Self> {
    type Output;

    fn Log(&self, base: &Base) -> Self::Output;
    fn lg() -> Self::Output;
    fn ln() -> Self::Output;
}
