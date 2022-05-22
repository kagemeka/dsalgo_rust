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

impl<M: Modulus> BinaryOperation<Multiplicative> for Modular<M> {
    type Codomain = Self;
    type Lhs = Self;
    type Rhs = Self;

    fn map(lhs: Self, rhs: Self) -> Self { lhs * rhs }
}

impl<M: Modulus> IdentityElement<Multiplicative> for Modular<M> {
    type X = Self;

    fn identity() -> Self { 1.into() }
}

impl<M: Modulus> AssociativeProperty<Multiplicative> for Modular<M> {}

impl<M: Modulus> MulInv for Modular<M> {
    type Output = Self;

    fn mul_inv(self) -> Self::Output { self.invert().unwrap() }
}
