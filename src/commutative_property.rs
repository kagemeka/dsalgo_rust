use crate::binary_operation::BinaryOperation;

pub trait CommutativeProperty<X, Y, Id>: BinaryOperation<X, X, Y, Id> {
    fn assert_commutative(a: X, b: X)
    where
        X: Clone,
        Y: PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::operate(a.clone(), b.clone()),
            Self::operate(b, a)
        );
    }
}

use crate::binary_operation::{BinaryOperation2, BinaryOperationId};

pub trait CommutativeProperty2<Id>:
    BinaryOperation2<Id, Lhs = Self::X, Rhs = Self::X>
where
    Id: BinaryOperationId,
{
    type X;

    fn assert_commutative(l: Self::X, r: Self::X)
    where
        Self::Codomain: PartialEq + std::fmt::Debug,
        Self::X: Clone,
    {
        assert_eq!(
            Self::operate(l.clone(), r.clone()),
            Self::operate(r, l)
        );
    }
}
