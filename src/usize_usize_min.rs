use crate::{
    associative_property::AssociativeProperty,
    binary_operation::BinaryOperation,
    group_theory_id::Min,
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
