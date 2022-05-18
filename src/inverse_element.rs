use crate::identity_element::IdentityElement;

pub trait InverseElement<X, Id>: IdentityElement<X, Id> {
    fn invert(element: X) -> X;
}

use crate::{
    binary_operation::BinaryOperationId,
    identity_element::IdentityElement2,
};

pub trait InverseElement2<Id>: IdentityElement2<Id>
where
    Id: BinaryOperationId,
{
    fn invert(element: Self::X) -> Self::X;
}
