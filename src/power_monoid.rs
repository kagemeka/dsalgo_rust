use crate::{monoid::Monoid, power_semigroup::pow_semigroup};

pub fn pow_monoid<S, M, Id>(x: S, exponent: u64) -> S
where
    S: Clone,
    M: Monoid<S, Id>,
{
    if exponent == 0 {
        M::identity()
    } else {
        pow_semigroup::<S, M, Id>(x, exponent)
    }
}

pub trait PowerMonoid<Id>: Monoid<Self, Id>
where
    Self: Clone,
{
    fn pow_monoid(self, exponent: u64) -> Self {
        pow_monoid::<Self, Self, Id>(self, exponent)
    }
}
impl<S, Id> PowerMonoid<Id> for S where S: Monoid<S, Id> + Clone {}
