use crate::{
    binary_operation::BinaryOperationId,
    commutative_property::CommutativeProperty,
    group::Group,
};

pub trait AbelianGroup<Id: BinaryOperationId>: Group<Id> {}

impl<Id, T> AbelianGroup<Id> for T
where
    T: Group<Id> + CommutativeProperty<Id>,
    Id: BinaryOperationId,
{
}
