use crate::binary_operation::{BinaryOperation, BinaryOperationId};

pub trait Magma<Id: BinaryOperationId> {
    type S;
    fn operate(l: Self::S, r: Self::S) -> Self::S;
}

impl<S, Id, T> Magma<Id> for T
where
    T: BinaryOperation<Id, Lhs = S, Rhs = S, Codomain = S>,
    Id: BinaryOperationId,
{
    type S = S;

    fn operate(l: Self::S, r: Self::S) -> Self::S { T::map(l, r) }
}
