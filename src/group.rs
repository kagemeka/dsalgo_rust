use crate::{
    binary_operation::BinaryOperationId,
    inverse_element::InverseElement,
    monoid::Monoid,
};

pub trait Group<Id: BinaryOperationId>: Monoid<Id> {
    fn invert(element: Self::S) -> Self::S;
}

impl<Id, T> Group<Id> for T
where
    T: Monoid<Id> + InverseElement<Id, X = Self::S>,
    Id: BinaryOperationId,
{
    fn invert(element: Self::S) -> Self::S { T::invert(element) }
}
