use crate::{
    associative_property::AssociativeProperty,
    binary_operation::BinaryOperationId,
    magma::Magma,
};

pub trait Semigroup<Id: BinaryOperationId>: Magma<Id> {}

impl<Id, T> Semigroup<Id> for T
where
    T: Magma<Id> + AssociativeProperty<Id>,
    Id: BinaryOperationId,
{
}
