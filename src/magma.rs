use crate::binary_operation::BinaryOperation;

pub trait Magma<S, Id>: BinaryOperation<S, S, S, Id> {}

impl<S, Id, T> Magma<S, Id> for T where T: BinaryOperation<S, S, S, Id> {}

use crate::binary_operation::{BinaryOperation2, BinaryOperationId};

pub trait Magma2<Id>:
    BinaryOperation2<Id, Lhs = Self::S, Rhs = Self::S, Codomain = Self::S>
where
    Id: BinaryOperationId,
{
    type S;
}

impl<S, Id, T> Magma2<Id> for T
where
    T: BinaryOperation2<Id, Lhs = S, Rhs = S, Codomain = S>,
    Id: BinaryOperationId,
{
    type S = S;
}
