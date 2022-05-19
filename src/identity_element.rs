pub fn is_left_identity<F, S>(f: &F, identity: S, x: S) -> bool
where
    F: Fn(S, S) -> S,
    S: Clone + PartialEq,
{
    f(identity, x.clone()) == x
}

pub fn is_right_identity<F, S>(f: &F, identity: S, x: S) -> bool
where
    F: Fn(S, S) -> S,
    S: Clone + PartialEq,
{
    f(x.clone(), identity) == x
}

pub fn is_identity<F, S>(f: &F, identity: S, x: S) -> bool
where
    F: Fn(S, S) -> S,
    S: Clone + PartialEq,
{
    is_left_identity(f, identity.clone(), x.clone())
        && is_right_identity(f, identity, x)
}

pub trait IdentityElement<X, Id> {
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
