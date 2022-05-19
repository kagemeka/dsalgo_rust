use crate::binary_operation::BinaryOperation;

pub fn is_commutative<F, X, Y>(f: &F, a: X, b: X) -> bool
where
    F: Fn(X, X) -> Y,
    X: Clone,
    Y: PartialEq,
{
    f(a.clone(), b.clone()) == f(b, a)
}

pub trait CommutativeProperty<X, Y, Id>: BinaryOperation<X, X, Y, Id> {}

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
