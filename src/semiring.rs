use crate::{
    binary_operation::BinaryOperationId,
    commutative_monoid::CommutativeMonoid,
    distributive_property::DistributiveProperty,
    magma::Magma,
    monoid::Monoid,
    zero_element::ZeroElement,
};

pub trait Semiring<Add, Mul>
where
    Add: BinaryOperationId,
    Mul: BinaryOperationId,
{
    type S;
    fn add(l: Self::S, r: Self::S) -> Self::S;
    fn mul(l: Self::S, r: Self::S) -> Self::S;
    fn zero() -> Self::S;
    fn one() -> Self::S;
}

impl<S, Add, Mul, T> Semiring<Add, Mul> for T
where
    T: CommutativeMonoid<Add, S = S>
        + Monoid<Mul, S = S>
        + DistributiveProperty<Add, Mul>
        + ZeroElement<Add, Mul>,
    Add: BinaryOperationId,
    Mul: BinaryOperationId,
{
    type S = S;

    fn add(l: Self::S, r: Self::S) -> Self::S {
        <T as Magma<Add>>::operate(l, r)
    }

    fn mul(l: Self::S, r: Self::S) -> Self::S {
        <T as Magma<Mul>>::operate(l, r)
    }

    fn zero() -> Self::S { <T as Monoid<Add>>::identity() }

    fn one() -> Self::S { <T as Monoid<Mul>>::identity() }
}
