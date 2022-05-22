use crate::{monoid::Monoid, segment_tree::SegmentTree};

impl<M, Id> SegmentTree<M, Id>
where
    M: Monoid<Id>,
    M::S: Clone,
{
    pub fn max_right<F>(&self, is_ok: &F, l: usize) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(l <= self.size);
        if l == self.size {
            return self.size;
        }
        let n = self.n();
        let mut v = M::identity();
        let mut i = l + n;
        debug_assert_ne!(i, 0);
        loop {
            i >>= i.trailing_zeros(); // upstream
            let nv = M::operate(v.clone(), self.data[i].clone());
            if !is_ok(&nv) {
                break;
            }
            // otherwise up one stair to right
            i += 1;
            v = nv;
            if i.count_ones() == 1 {
                return self.size;
            }
        }
        // down stairs to right
        while i < n {
            i <<= 1;
            let nv = M::operate(v.clone(), self.data[i].clone());
            if !is_ok(&nv) {
                continue;
            }
            v = nv;
            i += 1;
        }
        i - n
    }

    pub fn min_left<F>(&self, is_ok: &F, r: usize) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(r <= self.size);
        if r == 0 {
            return 0;
        }
        let n = self.n();
        let mut v = M::identity();
        let mut i = r + n;
        debug_assert_ne!(i, 0);
        loop {
            i >>= i.trailing_zeros(); // upstream
            let nv = M::operate(
                self.data[i - 1].clone(),
                v.clone(),
            );
            if !is_ok(&nv) {
                break;
            }
            i -= 1;
            v = nv;
            if i.count_ones() == 1 {
                return 0;
            }
        }
        while i < n {
            i <<= 1;
            let nv = M::operate(
                self.data[i - 1].clone(),
                v.clone(),
            );
            if !is_ok(&nv) {
                continue;
            }
            i -= 1;
            v = nv;
        }
        i - n
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
        seg.set(5, 10);
        let is_ok = &|sum: &usize| *sum < 10;
        assert_eq!(seg.max_right(is_ok, 0), 5);
        assert_eq!(seg.max_right(is_ok, 10), 10);
        assert_eq!(seg.max_right(is_ok, 5), 5);
        assert_eq!(seg.max_right(is_ok, 6), 10);

        assert_eq!(seg.min_left(is_ok, 10), 6);
        assert_eq!(seg.min_left(is_ok, 5), 0);
        assert_eq!(seg.min_left(is_ok, 6), 6);
    }
}
