use crate::binary_operation::BinaryOperation;

pub trait Magma<S, Id>: BinaryOperation<S, S, S, Id> {}
impl<S, Id, T> Magma<S, Id> for T where T: BinaryOperation<S, S, S, Id> {}
