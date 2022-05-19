use crate::{
    group::Group,
    power_monoid::pow_monoid,
    power_semigroup::pow_semigroup,
};

pub fn pow_group<F, E, Inv, X>(
    f: &F,
    e: &E,
    inv: &Inv,
    x: X,
    exponent: i64,
) -> X
where
    F: Fn(X, X) -> X,
    E: Fn() -> X,
    Inv: Fn(X) -> X,
    X: Clone,
{
    if exponent >= 0 {
        pow_monoid(f, e, x, exponent as u64)
    } else {
        pow_semigroup(f, inv(x), -exponent as u64)
    }
}

pub trait PowerGroup<Id>: Group<Id, S = Self>
where
    Self: Clone,
{
    fn pow_group(self, exponent: i64) -> Self {
        pow_group(
            &Self::operate,
            &Self::identity,
            &Self::invert,
            self,
            exponent,
        )
    }
}
impl<S, Id> PowerGroup<Id> for S where S: Group<Id, S = S> + Clone {}
