use crate::binary_operation::BinaryOperation;

pub trait IdentityElement<S, Id>: BinaryOperation<S, S, S, Id> {
    fn identity() -> S;
}
