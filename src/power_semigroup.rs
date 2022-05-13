use crate::semigroup::Semigroup;

pub fn pow_semigroup_recurse<S, Id, G>(x: S, exponent: u64) -> S
where
    S: Copy,
    G: Semigroup<S, Id>,
{
    assert!(exponent > 0);
    if exponent == 1 {
        return x;
    }
    let mut y = pow_semigroup_recurse::<S, Id, G>(x, exponent >> 1);
    y = G::operate(y, y);
    if exponent & 1 == 1 {
        y = G::operate(y, x);
    }
    y
}

pub fn pow_semigroup<S, Id, G>(mut x: S, mut exponent: u64) -> S
where
    S: Copy,
    G: Semigroup<S, Id>,
{
    assert!(exponent > 0);
    let mut y = x;
    exponent -= 1;
    while exponent > 0 {
        if exponent & 1 == 1 {
            y = G::operate(y, x);
        }
        x = G::operate(x, x);
        exponent >>= 1;
    }
    y
}
pub trait PowerSemigroup<Id>: Semigroup<Self, Id>
where
    Self: Copy,
{
    fn pow_seimigroup(self, exponent: u64) -> Self { pow_semigroup::<Self, Id, Self>(self, exponent) }
}

impl<S, Id> PowerSemigroup<Id> for S where S: Semigroup<S, Id> + Copy {}
