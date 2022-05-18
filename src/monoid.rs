use crate::{identity_element::IdentityElement, semigroup::Semigroup};

pub trait Monoid<S, Id>: Semigroup<S, Id> + IdentityElement<S, Id> {}

impl<S, Id, T> Monoid<S, Id> for T where
    T: Semigroup<S, Id> + IdentityElement<S, Id>
{
}

use crate::{
    binary_operation::BinaryOperationId,
    identity_element::IdentityElement2,
    semigroup::Semigroup2,
};

pub trait Monoid2<Id>:
    Semigroup2<Id> + IdentityElement2<Id, X = Self::S>
where
    Id: BinaryOperationId,
{
}

impl<Id, T> Monoid2<Id> for T
where
    T: Semigroup2<Id> + IdentityElement2<Id, X = Self::S>,
    Id: BinaryOperationId,
{
}
