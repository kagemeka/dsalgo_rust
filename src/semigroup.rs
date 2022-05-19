use crate::{associative_property::AssociativeProperty, magma::Magma};

pub trait Semigroup<S, Id>: Magma<S, Id> {}

impl<S, Id, T> Semigroup<S, Id> for T where
    T: Magma<S, Id> + AssociativeProperty<S, Id>
{
}

use crate::{
    associative_property::AssociativeProperty2,
    binary_operation::BinaryOperationId,
    magma::Magma2,
};

pub trait Semigroup2<Id>:
    Magma2<Id> + AssociativeProperty2<Id, X = Self::S>
where
    Id: BinaryOperationId,
{
}

impl<Id, T> Semigroup2<Id> for T
where
    T: Magma2<Id> + AssociativeProperty2<Id, X = Self::S>,
    Id: BinaryOperationId,
{
}
