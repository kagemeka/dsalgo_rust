use crate::{commutative_monoid::CommutativeMonoid, monoid::Monoid};

pub trait Semiring<S, Add, Mul>:
    CommutativeMonoid<S, Add> + Monoid<S, Mul>
{
}
impl<S, Add, Mul, T> Semiring<S, Add, Mul> for T where
    T: CommutativeMonoid<S, Add> + Monoid<S, Mul>
{
}
