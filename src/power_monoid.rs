use crate::{monoid::Monoid, power_semigroup::pow_semigroup};

pub fn pow_monoid<S, Id, M>(x: S, exponent: u64) -> S
where
    S: Copy,
    M: Monoid<S, Id>,
{
    if exponent == 0 {
        M::identity()
    } else {
        pow_semigroup::<S, Id, M>(x, exponent)
    }
}

pub trait PowerMonoid<Id>: Monoid<Self, Id>
where
    Self: Copy,
{
    fn pow_monoid(self, exponent: u64) -> Self { pow_monoid::<Self, Id, Self>(self, exponent) }
}
impl<S, Id> PowerMonoid<Id> for S where S: Monoid<S, Id> + Copy {}
