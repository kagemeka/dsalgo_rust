use crate::{commutative_property::CommutativeProperty, monoid::Monoid};

pub trait CommutativeMonoid<S, Id>:
    Monoid<S, Id> + CommutativeProperty<S, S, Id> + Sized
{
}
impl<S, Id, T> CommutativeMonoid<S, Id> for T where
    T: Monoid<S, Id> + CommutativeProperty<S, S, Id>
{
}

use crate::{
    binary_operation::BinaryOperationId,
    commutative_property::CommutativeProperty2,
    monoid::Monoid2,
};

pub trait CommutativeMonoid2<Id>:
    Monoid2<Id> + CommutativeProperty2<Id, X = Self::S>
where
    Id: BinaryOperationId,
{
}

impl<Id, T> CommutativeMonoid2<Id> for T
where
    T: Monoid2<Id> + CommutativeProperty2<Id, X = Self::S>,
    Id: BinaryOperationId,
{
}
