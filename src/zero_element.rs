use crate::binary_operation::BinaryOperationId;

pub trait ZeroElement<Add, Mul>
where
    Add: BinaryOperationId,
    Mul: BinaryOperationId,
{
}
