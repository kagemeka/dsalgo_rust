use crate::binary_operation::BinaryOperation;

pub trait AssociativeProperty<X, Id>: BinaryOperation<X, X, X, Id> {
    fn assert_associative(first: X, second: X, third: X)
    where
        X: Clone + PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::operate(
                Self::operate(first.clone(), second.clone()),
                third.clone()
            ),
            Self::operate(
                first,
                Self::operate(second, third)
            ),
        );
    }
}

use crate::binary_operation::{BinaryOperation2, BinaryOperationId};

pub trait AssociativeProperty2<Id>:
    BinaryOperation2<Id, Lhs = Self::X, Rhs = Self::X, Codomain = Self::X>
where
    Id: BinaryOperationId,
{
    type X;

    /// we don't return boolean result
    /// because it's validating rather than testing.
    /// users implementing this trait must assure
    /// that the operation is associative.
    fn assert_associative(first: Self::X, second: Self::X, third: Self::X)
    where
        Self::X: Clone + PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::operate(
                Self::operate(first.clone(), second.clone()),
                third.clone()
            ),
            Self::operate(
                first,
                Self::operate(second, third)
            ),
        );
    }
}
