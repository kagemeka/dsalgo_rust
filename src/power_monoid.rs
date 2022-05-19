use crate::{monoid::Monoid, power_semigroup::pow_semigroup};

pub fn pow_monoid<F, E, X>(f: &F, e: &E, x: X, exponent: u64) -> X
where
    F: Fn(X, X) -> X,
    E: Fn() -> X,
    X: Clone,
{
    if exponent == 0 { e() } else { pow_semigroup(f, x, exponent) }
}

pub trait PowerMonoid<Id>: Monoid<Id, S = Self>
where
    Self: Clone,
{
    fn pow_monoid(self, exponent: u64) -> Self {
        pow_monoid(
            &Self::operate,
            &Self::identity,
            self,
            exponent,
        )
    }
}

impl<S, Id> PowerMonoid<Id> for S where S: Monoid<Id, S = S> + Clone {}
