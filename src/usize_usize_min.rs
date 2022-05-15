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
    fn operate(lhs: (usize, usize), rhs: (usize, usize)) -> (usize, usize) {
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
