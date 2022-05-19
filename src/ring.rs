use crate::{commutative_monoid::CommutativeMonoid, monoid::Monoid};

pub trait Ring<S, Add, Mul>:
    CommutativeMonoid<S, Add> + Monoid<S, Mul>
{
}

impl<S, Add, Mul, T> Ring<S, Add, Mul> for T where
    T: CommutativeMonoid<S, Add> + Monoid<S, Mul>
{
}

use crate::{
    binary_operation::BinaryOperationId,
    inverse_element::InverseElement2,
    semiring::Semiring2,
};

pub trait Ring2<Add, Mul>: Semiring2<Add, Mul> + InverseElement2<Add>
where
    Add: BinaryOperationId,
    Mul: BinaryOperationId,
{
}

impl<Add, Mul, T> Ring2<Add, Mul> for T
where
    T: Semiring2<Add, Mul> + InverseElement2<Add>,
    Add: BinaryOperationId,
    Mul: BinaryOperationId,
{
}
