use crate::{commutative_monoid::CommutativeMonoid, monoid::Monoid};

pub trait Semiring<S, Add, Mul>:
    CommutativeMonoid<S, Add> + Monoid<S, Mul>
{
}

// TODO: + distributive + zero-element.
impl<S, Add, Mul, T> Semiring<S, Add, Mul> for T where
    T: CommutativeMonoid<S, Add> + Monoid<S, Mul>
{
}

use crate::{
    binary_operation::BinaryOperationId,
    commutative_monoid::CommutativeMonoid2,
    distributive_property::DistributiveProperty2,
    magma::Magma2,
    monoid::Monoid2,
    zero_element::ZeroElement,
};

pub trait Semiring2<Add, Mul>:
    CommutativeMonoid2<Add>
    + Monoid2<Mul, S = <Self as Magma2<Add>>::S>
    + DistributiveProperty2<Add, Mul, S = <Self as Magma2<Add>>::S>
    + ZeroElement<Add, Mul>
where
    Add: BinaryOperationId,
    Mul: BinaryOperationId,
{
}

impl<Add, Mul, T> Semiring2<Add, Mul> for T where
    T: CommutativeMonoid2<Add>
        + Monoid2<Mul, S = <Self as Magma2<Add>>::S>
        + DistributiveProperty2<Add, Mul, S = <Self as Magma2<Add>>::S>
        + ZeroElement<Add, Mul>
{
}
