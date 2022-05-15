use crate::{commutative_property::CommutativeProperty, group::Group};

pub trait AbelianGroup<S, Id>:
    Group<S, Id> + CommutativeProperty<S, S, Id>
{
}
impl<S, Id, T> AbelianGroup<S, Id> for T where
    T: Group<S, Id> + CommutativeProperty<S, S, Id>
{
}
