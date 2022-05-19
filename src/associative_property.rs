use crate::binary_operation::BinaryOperationId;

/// users implementing this trait must assure
/// that the operation for given id is associative.
pub trait AssociativeProperty<Id: BinaryOperationId> {}
