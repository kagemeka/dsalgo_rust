use crate::{associative_property::AssociativeProperty, magma::Magma};

pub trait Semigroup<S, Id>: Magma<S, Id> + AssociativeProperty<S, Id> {}

impl<S, Id, T> Semigroup<S, Id> for T where T: Magma<S, Id> + AssociativeProperty<S, Id> {}
