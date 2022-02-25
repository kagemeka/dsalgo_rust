use crate::{abstract_traits_2, power_2};

pub trait IsPrime {}

pub trait Modulo {
    const VALUE: usize;
}

pub struct Add;
pub struct Mul;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modular<M> {
    phantom: std::marker::PhantomData<M>,
    value: usize,
}

impl<M> std::fmt::Display for Modular<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl<M> Modular<M> {
    pub const fn value(&self) -> usize { self.value }
}

impl<M: Modulo> Modular<M> {
    pub fn new(n: usize) -> Self {
        Self {
            phantom: std::marker::PhantomData,
            value: n % M::VALUE,
        }
    }
}

impl<M: Modulo> From<usize> for Modular<M> {
    fn from(value: usize) -> Self { Self::new(value) }
}

impl<M: Modulo> abstract_traits_2::Identity<Self, Add> for Modular<M> {
    fn identity() -> Self { 0.into() }
}

impl<M: Modulo> abstract_traits_2::Identity<Self, Mul> for Modular<M> {
    fn identity() -> Self { 1.into() }
}

impl<M: Modulo> abstract_traits_2::BinaryOperation<Self, Add> for Modular<M> {
    fn operate(lhs: &Self, rhs: &Self) -> Self {
        (lhs.value + rhs.value).into()
    }
}

impl<M: Modulo> abstract_traits_2::BinaryOperation<Self, Mul> for Modular<M> {
    fn operate(lhs: &Self, rhs: &Self) -> Self {
        (lhs.value * rhs.value).into()
    }
}

impl<M: Modulo + Copy> std::ops::AddAssign<Self> for Modular<M> {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl<M: Modulo> std::ops::Add<Self> for Modular<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { (self.value + rhs.value).into() }
}

impl<M: Modulo> std::ops::Neg for Modular<M> {
    type Output = Self;

    fn neg(self) -> Self::Output { (M::VALUE - self.value).into() }
}

impl<M: Modulo + Copy> std::ops::SubAssign<Self> for Modular<M> {
    fn sub_assign(&mut self, rhs: Self) { *self += -rhs; }
}

impl<M: Modulo> std::ops::Sub<Self> for Modular<M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self { self + -rhs }
}

impl<M: Modulo> std::ops::Mul<Self> for Modular<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self { (self.value * rhs.value).into() }
}

impl<M: Modulo + Copy> std::ops::MulAssign<Self> for Modular<M> {
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

impl<M: Modulo + IsPrime> Modular<M> {
    pub fn invert(&self) -> Self {
        <Self as power_2::Power<Self, Mul>>::pow(self, M::VALUE - 2)
    }
}

impl<M: Modulo + IsPrime> std::ops::Div<Self> for Modular<M> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self { self * rhs.invert() }
}

impl<M: Modulo + IsPrime + Copy> std::ops::DivAssign<Self> for Modular<M> {
    fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MOD998_244_353;
impl Modulo for MOD998_244_353 {
    const VALUE: usize = 998_244_353;
}
impl IsPrime for MOD998_244_353 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MOD1_000_000_007;
impl Modulo for MOD1_000_000_007 {
    const VALUE: usize = 1_000_000_007;
}
impl IsPrime for MOD1_000_000_007 {}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        type Mint = super::Modular<super::MOD998_244_353>;

        let mut x = Mint::new(998_244_353);
        assert_eq!(x.value(), 0);
        x += Mint::new(10);
        assert_eq!(x.value(), 10);
        x -= Mint::new(9);
        assert_eq!(x.value(), 1);
        x /= Mint::new(2);
        assert_eq!(x.value(), 499122177);
    }
}
