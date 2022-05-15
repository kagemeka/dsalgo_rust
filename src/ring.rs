use crate::{identity_element::IdentityElement, semiring::Semiring};

pub trait Ring<S, Add, Mul>:
    Semiring<S, Add, Mul> + IdentityElement<S, Add>
{
}
impl<S, Add, Mul, T> Ring<S, Add, Mul> for T where
    T: Semiring<S, Add, Mul> + IdentityElement<S, Add>
{
}
