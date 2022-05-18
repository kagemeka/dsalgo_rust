use crate::binary_operation::BinaryOperation;

pub trait Idempotence<X, Id>: BinaryOperation<X, X, X, Id> {
    fn assert_idempotent(element: X)
    where
        X: Clone + PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::operate(
                element.clone(),
                element.clone()
            ),
            element
        );
    }
}

use crate::binary_operation::{BinaryOperation2, BinaryOperationId};

pub trait Idempotence2<Id>:
    BinaryOperation2<Id, Lhs = Self::X, Rhs = Self::X, Codomain = Self::X>
where
    Id: BinaryOperationId,
{
    type X;
    fn assert_idempotent(element: Self::X)
    where
        Self::X: Clone + PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::operate(
                element.clone(),
                element.clone()
            ),
            element
        );
    }
}
