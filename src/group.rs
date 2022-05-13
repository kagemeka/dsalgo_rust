use crate::{inverse_element::InverseElement, monoid::Monoid};

pub trait Group<S, Id>: Monoid<S, Id> + InverseElement<S, Id> {}

impl<S, Id, T> Group<S, Id> for T where T: Monoid<S, Id> + InverseElement<S, Id> {}
