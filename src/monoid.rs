use crate::{
    binary_operation::BinaryOperationId,
    identity_element::IdentityElement,
    semigroup::Semigroup,
};

pub trait Monoid<Id: BinaryOperationId>: Semigroup<Id> {
    fn identity() -> Self::S;
}

impl<Id, T> Monoid<Id> for T
where
    T: Semigroup<Id> + IdentityElement<Id, X = Self::S>,
    Id: BinaryOperationId,
{
    fn identity() -> Self::S { T::identity() }
}
