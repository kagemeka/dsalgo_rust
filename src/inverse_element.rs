use crate::identity_element::IdentityElement;

pub trait InverseElement<S, Id>: IdentityElement<S, Id> {
    fn invert(element: S) -> S;
}
