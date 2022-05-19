use crate::binary_operation::BinaryOperationId;

pub trait InverseElement<Id: BinaryOperationId> {
    type X;
    fn invert(element: Self::X) -> Self::X;
}
