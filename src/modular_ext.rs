use crate::{
    associative_property::AssociativeProperty,
    binary_operation::BinaryOperation,
    group_theory_id::Multiplicative,
    identity_element::IdentityElement,
    modular::Modular,
    modulus::Modulus,
    power_monoid::PowerMonoid,
};

impl<M: Modulus> BinaryOperation<Self, Self, Self, Multiplicative> for Modular<M> {
    fn operate(lhs: Self, rhs: Self) -> Self { lhs * rhs }
}

impl<M: Modulus> IdentityElement<Self, Multiplicative> for Modular<M> {
    fn identity() -> Self { 1.into() }
}

impl<M: Modulus> AssociativeProperty<Self, Multiplicative> for Modular<M> {}

impl<M: Modulus + std::marker::Copy> Modular<M> {
    pub fn pow(self, exponent: u64) -> Self { self.pow_monoid(exponent) }
}
