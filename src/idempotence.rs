use crate::binary_operation::BinaryOperation;

pub fn is_idempotent<F, X>(f: &F, x: X) -> bool
where
    F: Fn(X, X) -> X,
    X: Clone + PartialEq,
{
    f(x.clone(), x.clone()) == x
}

pub trait Idempotence<X, Id>: BinaryOperation<X, X, X, Id> {}

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
