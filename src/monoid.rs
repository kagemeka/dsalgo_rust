use crate::{identity_element::IdentityElement, semigroup::Semigroup};

pub trait Monoid<S, Id>: Semigroup<S, Id> + IdentityElement<S, Id> {}
impl<S, Id, T> Monoid<S, Id> for T where
    T: Semigroup<S, Id> + IdentityElement<S, Id>
{
}
