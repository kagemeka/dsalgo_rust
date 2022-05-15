use crate::{
    associative_property::AssociativeProperty,
    binary_operation::BinaryOperation,
    group_theory_id::Multiplicative,
    identity_element::IdentityElement,
    modular::Modular,
    modulus::Modulus,
    multiplicative_inverse::MulInv,
    power_monoid::PowerMonoid,
};

impl<M: Modulus> From<i32> for Modular<M> {
    fn from(value: i32) -> Self { Self::from(value as i64) }
}

impl<M: Modulus> From<usize> for Modular<M> {
    fn from(value: usize) -> Self { Self::from(value as u64) }
}

impl<M: Modulus> BinaryOperation<Self, Self, Self, Multiplicative>
    for Modular<M>
{
    fn operate(lhs: Self, rhs: Self) -> Self { lhs * rhs }
}

impl<M: Modulus> IdentityElement<Self, Multiplicative> for Modular<M> {
    fn identity() -> Self { 1.into() }
}

impl<M: Modulus> AssociativeProperty<Self, Multiplicative> for Modular<M> {}

impl<M: Modulus + Clone> Modular<M> {
    pub fn pow(self, exponent: u64) -> Self { self.pow_monoid(exponent) }
}

impl<M: Modulus> MulInv for Modular<M> {
    type Output = Self;

    fn mul_inv(self) -> Self::Output { self.invert().unwrap() }
}
