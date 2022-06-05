use crate::{
    associative_property::AssociativeProperty,
    binary_operation::BinaryOperation,
    group_theory_id::Multiplicative,
    identity_element::IdentityElement,
    modular_int::ModularInt,
    modulus::Modulus,
    multiplicative_inverse::MulInv,
};

impl<M: Modulus> From<i32> for ModularInt<M> {
    fn from(value: i32) -> Self { Self::from(value as i64) }
}

impl<M: Modulus> From<usize> for ModularInt<M> {
    fn from(value: usize) -> Self { Self::from(value as u64) }
}

impl<M: Modulus> BinaryOperation<Multiplicative> for ModularInt<M> {
    type Codomain = Self;
    type Lhs = Self;
    type Rhs = Self;

    fn map(lhs: Self, rhs: Self) -> Self { lhs * rhs }
}

impl<M: Modulus> IdentityElement<Multiplicative> for ModularInt<M> {
    type X = Self;

    fn identity() -> Self { 1.into() }
}

impl<M: Modulus> AssociativeProperty<Multiplicative> for ModularInt<M> {}

impl<M: Modulus> MulInv for ModularInt<M> {
    type Output = Self;

    fn mul_inv(self) -> Self::Output { self.invert().unwrap() }
}
