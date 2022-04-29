use crate::{group_theory, power};

pub trait Modulus {
    fn value() -> usize;
}

pub trait IsPrime: Modulus {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modular<M: Modulus> {
    phantom: std::marker::PhantomData<M>,
    value: usize,
}

impl<M: Modulus> std::fmt::Display for Modular<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl<M: Modulus> Modular<M> {
    pub const fn value(&self) -> usize { self.value }
}

impl<M: Modulus> Modular<M> {
    pub fn new(n: usize) -> Self {
        Self {
            phantom: std::marker::PhantomData,
            value: n % M::value(),
        }
    }
}

impl<M: Modulus> From<usize> for Modular<M> {
    fn from(value: usize) -> Self { Self::new(value) }
}

impl<M: Modulus + std::marker::Copy> group_theory::IdentityElement<group_theory::Additive>
    for Modular<M>
{
    fn identity() -> Self { 0.into() }
}

impl<M: Modulus + std::marker::Copy> group_theory::IdentityElement<group_theory::Multiplicative>
    for Modular<M>
{
    fn identity() -> Self { 1.into() }
}

// impl<M: Modulus> group_theory::Associative<Mul> for
// Modular<M> {}

impl<M: Modulus + Copy> std::ops::AddAssign<Self> for Modular<M> {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl<M: Modulus> std::ops::Add<Self> for Modular<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { (self.value + rhs.value).into() }
}

impl<M: Modulus> std::ops::Neg for Modular<M> {
    type Output = Self;

    fn neg(self) -> Self::Output { (M::value() - self.value).into() }
}

impl<M: Modulus + Copy> std::ops::SubAssign<Self> for Modular<M> {
    fn sub_assign(&mut self, rhs: Self) { *self += -rhs; }
}

impl<M: Modulus> std::ops::Sub<Self> for Modular<M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self { self + -rhs }
}

impl<M: Modulus> std::ops::Mul<Self> for Modular<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self { (self.value * rhs.value).into() }
}

impl<M: Modulus + Copy> std::ops::MulAssign<Self> for Modular<M> {
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

impl<M: Modulus + std::marker::Copy> Modular<M> {
    pub fn pow(&self, exponent: usize) -> Self {
        <Self as power::Power<group_theory::Multiplicative>>::pow(self.clone(), exponent)
    }
}

impl<M: Modulus + IsPrime + std::marker::Copy> Modular<M> {
    pub fn invert(&self) -> Self { self.pow(M::value() - 2) }
}

impl<M: Modulus + IsPrime + std::marker::Copy> std::ops::Div<Self> for Modular<M> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self { self * rhs.invert() }
}

impl<M: Modulus + IsPrime + Copy> std::ops::DivAssign<Self> for Modular<M> {
    fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}
