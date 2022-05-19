use crate::binary_operation::BinaryOperationId;

pub trait DistributiveProperty<Add, Mul>
where
    Add: BinaryOperationId,
    Mul: BinaryOperationId,
{
}
