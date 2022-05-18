use crate::{commutative_property::CommutativeProperty, group::Group};

pub trait AbelianGroup<S, Id>:
    Group<S, Id> + CommutativeProperty<S, S, Id>
{
}

impl<S, Id, T> AbelianGroup<S, Id> for T where
    T: Group<S, Id> + CommutativeProperty<S, S, Id>
{
}

use crate::{
    binary_operation::BinaryOperationId,
    commutative_property::CommutativeProperty2,
    group::Group2,
};

pub trait AbelianGroup2<Id>:
    Group2<Id> + CommutativeProperty2<Id, X = Self::S>
where
    Id: BinaryOperationId,
{
}

impl<Id, T> AbelianGroup2<Id> for T
where
    T: Group2<Id> + CommutativeProperty2<Id, X = Self::S>,
    Id: BinaryOperationId,
{
}
