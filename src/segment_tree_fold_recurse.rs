use crate::{monoid::Monoid, segment_tree::SegmentTree};

impl<S, Id, M> SegmentTree<S, Id, M>
where
    S: Clone,
    M: Monoid<S, Id>,
{
    pub fn fold_recurse(&self, l: usize, r: usize) -> S {
        assert!(l <= r && r <= self.size);
        self._fold_recurse(l, r, 0, self.n(), 1)
    }

    fn _fold_recurse(
        &self,
        l: usize,
        r: usize,
        cur_l: usize,
        cur_r: usize,
        i: usize,
    ) -> S {
        if cur_r <= l || r <= cur_l {
            return M::identity();
        }
        if l <= cur_l && cur_r <= r {
            return self.data[i].clone();
        }
        let c = (cur_l + cur_r) >> 1;
        M::operate(
            self._fold_recurse(l, r, cur_l, c, i << 1),
            self._fold_recurse(l, r, c, cur_r, i << 1 | 1),
        )
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
        assert_eq!(seg.fold_recurse(0, 10), 0);
        seg.set(5, 5);
        assert_eq!(seg.fold_recurse(0, 10), 5);
        seg.set(5, 10);
        assert_eq!(seg.fold_recurse(0, 10), 10);
    }
}
