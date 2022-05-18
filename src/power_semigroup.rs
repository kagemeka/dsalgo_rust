use crate::semigroup::Semigroup;

pub fn pow_semigroup_recurse<S, G, Id>(x: S, exponent: u64) -> S
where
    S: Clone,
    G: Semigroup<S, Id>,
{
    assert!(exponent > 0);
    if exponent == 1 {
        return x;
    }
    let mut y = pow_semigroup_recurse::<S, G, Id>(x.clone(), exponent >> 1);
    y = G::operate(y.clone(), y);
    if exponent & 1 == 1 {
        y = G::operate(y, x);
    }
    y
}

pub fn pow_semigroup<S, G, Id>(mut x: S, mut exponent: u64) -> S
where
    G: Semigroup<S, Id>,
    S: Clone,
{
    assert!(exponent > 0);
    let mut y = x.clone();
    exponent -= 1;
    while exponent > 0 {
        if exponent & 1 == 1 {
            y = G::operate(y, x.clone());
        }
        x = G::operate(x.clone(), x.clone());
        exponent >>= 1;
    }
    y
}

pub trait PowerSemigroup<Id>: Semigroup<Self, Id>
where
    Self: Clone,
{
    fn pow_seimigroup(self, exponent: u64) -> Self {
        pow_semigroup::<Self, Self, Id>(self, exponent)
    }
}
impl<S, Id> PowerSemigroup<Id> for S where S: Semigroup<S, Id> + Clone {}
