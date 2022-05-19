use crate::{monoid::Monoid, segment_tree::SegmentTree};

impl<M, Id> SegmentTree<M, Id>
where
    M: Monoid<Id>,
    M::S: Clone,
{
    pub fn reduce_recurse(&self, l: usize, r: usize) -> M::S {
        assert!(l <= r && r <= self.size);
        self._reduce_recurse(l, r, 0, self.n(), 1)
    }

    fn _reduce_recurse(
        &self,
        l: usize,
        r: usize,
        cur_l: usize,
        cur_r: usize,
        i: usize,
    ) -> M::S {
        if cur_r <= l || r <= cur_l {
            return M::identity();
        }
        if l <= cur_l && cur_r <= r {
            return self.data[i].clone();
        }
        let c = (cur_l + cur_r) >> 1;
        M::operate(
            self._reduce_recurse(l, r, cur_l, c, i << 1),
            self._reduce_recurse(l, r, c, cur_r, i << 1 | 1),
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
        assert_eq!(seg.reduce_recurse(0, 10), 0);
        seg.set(5, 5);
        assert_eq!(seg.reduce_recurse(0, 10), 5);
        seg.set(5, 10);
        assert_eq!(seg.reduce_recurse(0, 10), 10);
    }
}
