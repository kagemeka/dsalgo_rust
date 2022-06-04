use crate::{
    associative_property::AssociativeProperty,
    binary_operation::BinaryOperation,
    commutative_property::CommutativeProperty,
    gcd_00_is_0::gcd_00_is_0,
    group_theory_id::GCD,
    idempotence::Idempotence,
    identity_element::IdentityElement,
};

impl BinaryOperation<GCD> for u64 {
    type Codomain = Self;
    type Lhs = Self;
    type Rhs = Self;

    fn map(l: Self, r: Self) -> Self { gcd_00_is_0(l, r) }
}
impl AssociativeProperty<GCD> for u64 {}
impl CommutativeProperty<GCD> for u64 {}
impl Idempotence<GCD> for u64 {}
impl IdentityElement<GCD> for u64 {
    type X = Self;

    fn identity() -> Self { 0 }
}
