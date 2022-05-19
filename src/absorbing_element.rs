use crate::binary_operation::BinaryOperationId;

pub trait AbsorbingElement<Id: BinaryOperationId> {
    type X;
    fn absorbing_element() -> Self::X;
}
