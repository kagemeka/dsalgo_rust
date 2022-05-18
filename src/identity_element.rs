use crate::binary_operation::BinaryOperation;

pub trait IdentityElement<X, Id>: BinaryOperation<X, X, X, Id> {
    fn identity() -> X;
}

use crate::binary_operation::{BinaryOperation2, BinaryOperationId};

// TODO: define left, right identity and inherit them.
pub trait IdentityElement2<Id>:
    BinaryOperation2<Id, Lhs = Self::X, Rhs = Self::X, Codomain = Self::X>
where
    Id: BinaryOperationId,
{
    type X;
    fn identity() -> Self::X;

    fn assert_identity(element: Self::X)
    where
        Self::X: Clone + PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::operate(
                Self::identity(),
                element.clone(),
            ),
            element
        );
        assert_eq!(
            Self::operate(
                element.clone(),
                Self::identity()
            ),
            element
        );
    }
}
