use crate::{inverse_element::InverseElement, monoid::Monoid};

pub trait Group<S, Id>: Monoid<S, Id> + InverseElement<S, Id> {}

impl<S, Id, T: Monoid<S, Id> + InverseElement<S, Id>> Group<S, Id> for T {}

use crate::{
    binary_operation::BinaryOperationId,
    inverse_element::InverseElement2,
    monoid::Monoid2,
};

pub trait Group2<Id>: Monoid2<Id> + InverseElement2<Id, X = Self::S>
where
    Id: BinaryOperationId,
{
}

impl<Id, T> Group2<Id> for T
where
    T: Monoid2<Id> + InverseElement2<Id, X = Self::S>,
    Id: BinaryOperationId,
{
}
