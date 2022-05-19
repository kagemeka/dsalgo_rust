use crate::binary_operation::BinaryOperationId;
pub trait IdentityElement<Id: BinaryOperationId> {
    type X;
    fn identity() -> Self::X;
}
