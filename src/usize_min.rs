use crate::{
    associative_property::AssociativeProperty,
    binary_operation::BinaryOperation,
    commutative_property::CommutativeProperty,
    group_theory_id::Min,
    idempotence::Idempotence,
    identity_element::IdentityElement,
};

impl BinaryOperation<Min> for (usize, usize) {
    type Codomain = Self;
    type Lhs = Self;
    type Rhs = Self;

    fn map(lhs: Self, rhs: Self) -> Self { std::cmp::min(lhs, rhs) }
}

impl AssociativeProperty<Min> for (usize, usize) {}
impl IdentityElement<Min> for (usize, usize) {
    type X = Self;

    fn identity() -> Self {
        (
            std::usize::MAX,
            std::usize::MAX,
        )
    }
}

impl CommutativeProperty<Min> for (usize, usize) {}
impl Idempotence<Min> for (usize, usize) {}
