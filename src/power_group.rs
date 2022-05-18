use crate::{group::Group, power_monoid::pow_monoid};

pub fn pow_group<S, G, Id>(x: S, exponent: i64) -> S
where
    S: Clone,
    G: Group<S, Id>,
{
    if exponent >= 0 {
        pow_monoid::<S, G, Id>(x, exponent as u64)
    } else {
        pow_monoid::<S, G, Id>(G::invert(x), -exponent as u64)
    }
}

pub trait PowerGroup<Id>: Group<Self, Id>
where
    Self: Clone,
{
    fn pow_group(self, exponent: i64) -> Self {
        pow_group::<Self, Self, Id>(self, exponent)
    }
}
impl<S, Id> PowerGroup<Id> for S where S: Group<S, Id> + Clone {}
