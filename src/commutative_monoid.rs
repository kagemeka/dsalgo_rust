use crate::{commutative_property::CommutativeProperty, monoid::Monoid};

pub trait CommutativeMonoid<S, Id>:
    Monoid<S, Id> + CommutativeProperty<S, S, Id> + Sized
{
}
impl<S, Id, T> CommutativeMonoid<S, Id> for T where
    T: Monoid<S, Id> + CommutativeProperty<S, S, Id>
{
}
