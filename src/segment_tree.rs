use std::iter::FromIterator;

use crate::monoid::Monoid;

pub struct SegmentTree<S, Id, M>
where
    M: Monoid<S, Id>,
{
    phantom_id: std::marker::PhantomData<Id>,
    phantom_m: std::marker::PhantomData<M>,
    pub(crate) size: usize,
    pub(crate) data: Vec<S>,
}

impl<S, Id, M> std::iter::FromIterator<S> for SegmentTree<S, Id, M>
where
    S: Clone,
    M: Monoid<S, Id>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let mut data = iter.into_iter().collect::<Vec<_>>();
        let size = data.len();
        let n = size.next_power_of_two();
        data = (0..n)
            .map(|_| M::identity())
            .chain(data.into_iter())
            .chain((0..n - size).map(|_| M::identity()))
            .collect::<Vec<_>>();
        let mut seg = Self {
            phantom_id: std::marker::PhantomData,
            phantom_m: std::marker::PhantomData,
            size,
            data,
        };
        (1..n).rev().for_each(|i| seg.update(i));
        seg
    }
}

impl<S, Id, M> SegmentTree<S, Id, M>
where
    M: Monoid<S, Id>,
{
    pub fn size(&self) -> usize { self.size }

    pub(crate) fn n(&self) -> usize { self.data.len() >> 1 }
}

impl<S, Id, M> SegmentTree<S, Id, M>
where
    S: Clone,
    M: Monoid<S, Id>,
{
    pub fn new<F>(size: usize, default: F) -> Self
    where
        F: Fn() -> S,
    {
        Self::from_iter((0..size).map(|_| default()))
    }

    pub(crate) fn update(&mut self, i: usize) {
        self.data[i] = M::operate(
            self.data[i << 1].clone(),
            self.data[i << 1 | 1].clone(),
        );
    }

    pub fn set(&mut self, mut i: usize, x: S) {
        assert!(i < self.size);
        i += self.n();
        self.data[i] = x;
        while i > 1 {
            i >>= 1;
            self.update(i);
        }
    }

    pub fn fold(&self, mut l: usize, mut r: usize) -> S {
        assert!(l <= r && r <= self.size);
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
        impl BinaryOperation<usize, usize, usize, Additive> for Mon {
            fn operate(x: usize, y: usize) -> usize { x + y }
        }
        impl AssociativeProperty<usize, Additive> for Mon {}
        impl IdentityElement<usize, Additive> for Mon {
            fn identity() -> usize { 0 }
        }
        let mut seg = super::SegmentTree::<_, _, Mon>::new(10, || 0);
        assert_eq!(seg.fold(0, 10), 0);
        seg.set(5, 5);
        assert_eq!(seg.fold(0, 10), 5);
        seg.set(5, 10);
        assert_eq!(seg.fold(0, 10), 10);
    }
}
