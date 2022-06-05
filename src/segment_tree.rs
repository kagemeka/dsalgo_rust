// TODO: review group theory trait constraints.
// is it needed to be monoid to call `update`, `set`, ...?
// answer: No.

use std::iter::FromIterator;

use crate::monoid::Monoid;

pub struct SegmentTree<M: Monoid<Id>, Id> {
    pub(crate) size: usize,
    pub(crate) data: Vec<M::S>,
}

impl<M, Id> std::iter::FromIterator<M::S> for SegmentTree<M, Id>
where
    M: Monoid<Id>,
    M::S: Clone,
{
    fn from_iter<T: IntoIterator<Item = M::S>>(iter: T) -> Self {
        let mut data = iter.into_iter().collect::<Vec<_>>();
        let size = data.len();
        let n = size.next_power_of_two();
        data = (0..n)
            .map(|_| M::identity())
            .chain(data.into_iter())
            .chain((0..n - size).map(|_| M::identity()))
            .collect::<Vec<_>>();
        let mut seg = Self { size, data };
        (1..n).rev().for_each(|i| seg.update(i));
        seg
    }
}

impl<M: Monoid<Id>, Id> SegmentTree<M, Id> {
    pub fn size(&self) -> usize { self.size }

    pub(crate) fn n(&self) -> usize { self.data.len() >> 1 }
}

impl<M, Id> SegmentTree<M, Id>
where
    M: Monoid<Id>,
    M::S: Clone,
{
    pub fn new<F>(size: usize, default: F) -> Self
    where
        F: Fn() -> M::S,
    {
        Self::from_iter((0..size).map(|_| default()))
    }

    fn update(&mut self, i: usize) {
        self.data[i] = M::operate(
            self.data[i << 1].clone(),
            self.data[i << 1 | 1].clone(),
        );
    }

    pub fn set(&mut self, mut i: usize, x: M::S) {
        assert!(i < self.size);
        i += self.n();
        self.data[i] = x;
        while i > 1 {
            i >>= 1;
            self.update(i);
        }
    }

    /// not `reduce` but `fold`?
    /// but initial element internally is just the identity element.
    /// it's not an arbitrary element.
    /// also, it is not necessarily used to compute _{op}\prod_l^r (l < r).
    /// we use the identity only to make it easy implementation.
    /// (requireing monoid for simplicity,
    /// however, strictly, it's enough to be only semigrouop.)
    /// so this method should be called `reduce`.
    pub fn reduce(&self, mut l: usize, mut r: usize) -> M::S {
        assert!(l < r && r <= self.size);
        let n = self.n();
        l += n;
        r += n;
        let mut vl = M::identity();
        let mut vr = M::identity();
        while l < r {
            if l & 1 == 1 {
                vl = M::operate(vl, self.data[l].clone());
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = M::operate(self.data[r].clone(), vr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::operate(vl, vr)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_as_monoid() {
        // use crate::monoid::Monoid;
        use crate::{
            associative_property::AssociativeProperty,
            binary_operation::BinaryOperation,
            group_theory_id::Additive,
            identity_element::IdentityElement,
        };
        struct Mon;
        impl BinaryOperation<Additive> for Mon {
            type Codomain = usize;
            type Lhs = usize;
            type Rhs = usize;

            fn map(x: usize, y: usize) -> usize { x + y }
        }
        impl AssociativeProperty<Additive> for Mon {}
        impl IdentityElement<Additive> for Mon {
            type X = usize;

            fn identity() -> usize { 0 }
        }
        let mut seg = super::SegmentTree::<Mon, _>::new(10, || 0);
        assert_eq!(seg.reduce(0, 10), 0);
        seg.set(5, 5);
        assert_eq!(seg.reduce(0, 10), 5);
        seg.set(5, 10);
        assert_eq!(seg.reduce(0, 10), 10);
    }
}
