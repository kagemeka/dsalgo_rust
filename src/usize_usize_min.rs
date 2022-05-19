use crate::{
    associative_property::AssociativeProperty,
    binary_operation::BinaryOperation,
    commutative_property::CommutativeProperty,
    group_theory_id::Min,
    idempotence::Idempotence,
    identity_element::IdentityElement,
};

impl BinaryOperation<(usize, usize), (usize, usize), (usize, usize), Min>
    for (usize, usize)
{
    fn map(lhs: (usize, usize), rhs: (usize, usize)) -> (usize, usize) {
        std::cmp::min(lhs, rhs)
    }
}

impl AssociativeProperty<(usize, usize), Min> for (usize, usize) {}

impl IdentityElement<(usize, usize), Min> for (usize, usize) {
    fn identity() -> (usize, usize) {
        (
            std::usize::MAX,
            std::usize::MAX,
        )
    }
}

impl CommutativeProperty<(usize, usize), (usize, usize), Min>
    for (usize, usize)
{
}

impl Idempotence<(usize, usize), Min> for (usize, usize) {}

use crate::{
    associative_property::AssociativeProperty2,
    binary_operation::BinaryOperation2,
    commutative_property::CommutativeProperty2,
    idempotence::Idempotence2,
    identity_element::IdentityElement2,
};

impl BinaryOperation2<Min> for (usize, usize) {
    type Codomain = Self;
    type Lhs = Self;
    type Rhs = Self;

    fn operate(lhs: Self::Lhs, rhs: Self::Rhs) -> Self::Codomain {
        std::cmp::min(lhs, rhs)
    }
}

impl AssociativeProperty2<Min> for (usize, usize) {
    type X = Self;
}

impl IdentityElement2<Min> for (usize, usize) {
    type X = Self;

    fn identity() -> Self::X {
        (
            std::usize::MAX,
            std::usize::MAX,
        )
    }
}

impl CommutativeProperty2<Min> for (usize, usize) {
    type X = Self;
}

impl Idempotence2<Min> for (usize, usize) {
    type X = Self;
}
