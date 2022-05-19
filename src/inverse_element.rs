pub fn has_inverse<F, X>(f: &F, x: X) -> bool
where
    F: Fn(X) -> X,
    X: Clone + PartialEq,
{
    f(f(x.clone())) == x
}

pub trait InverseElement<X, Id> {
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
