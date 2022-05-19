use crate::{monoid::Monoid, segment_tree::SegmentTree};

impl<M, Id> SegmentTree<M, Id>
where
    M: Monoid<Id>,
    M::S: Clone,
{
    pub fn max_right_recurse<F>(&self, is_ok: &F, l: usize) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(l <= self.size);
        self._max_right_recurse(
            is_ok,
            l,
            0,
            self.n(),
            &mut M::identity(),
            1,
        )
    }

    /// find max right satisfying current_left <= right <= current_right.
    /// if current_right <= left, return left
    /// if current_left >= self.size, return self.size
    fn _max_right_recurse<F>(
        &self,
        is_ok: &F,
        l: usize,
        cur_l: usize,
        cur_r: usize,
        v: &mut M::S,
        i: usize,
    ) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        if cur_r <= l {
            return l;
        }
        if cur_l >= self.size {
            return self.size;
        }
        let nv = M::operate(v.clone(), self.data[i].clone());
        if l <= cur_l && cur_r <= self.size && is_ok(&nv) {
            *v = nv;
            return cur_r;
        }
        if cur_r - cur_l == 1 {
            return cur_l;
        }
        let c = (cur_l + cur_r) >> 1;
        let res = self._max_right_recurse(is_ok, l, cur_l, c, v, i << 1);
        if res < c || res == self.size {
            return res;
        }
        self._max_right_recurse(
            is_ok,
            l,
            c,
            cur_r,
            v,
            i << 1 | 1,
        )
    }

    pub fn min_left_recurse<F>(&self, is_ok: &F, r: usize) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(r <= self.size);
        self._min_left_recurse(
            is_ok,
            r,
            0,
            self.n(),
            &mut M::identity(),
            1,
        )
    }

    fn _min_left_recurse<F>(
        &self,
        is_ok: &F,
        r: usize,
        cur_l: usize,
        cur_r: usize,
        v: &mut M::S,
        i: usize,
    ) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        if cur_l >= r {
            return r;
        }
        let nv = M::operate(self.data[i].clone(), v.clone());
        if cur_r <= r && is_ok(&nv) {
            *v = nv;
            return cur_l;
        }
        if cur_r - cur_l == 1 {
            return cur_r;
        }
        let c = (cur_l + cur_r) >> 1;
        let res = self._min_left_recurse(
            is_ok,
            r,
            c,
            cur_r,
            v,
            i << 1 | 1,
        );
        if res > c {
            return res;
        }
        self._min_left_recurse(is_ok, r, cur_l, c, v, i << 1)
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
        assert_eq!(
            seg.max_right_recurse(is_ok, 0),
            5
        );
        assert_eq!(
            seg.max_right_recurse(is_ok, 10),
            10
        );
        assert_eq!(
            seg.max_right_recurse(is_ok, 5),
            5
        );
        assert_eq!(
            seg.max_right_recurse(is_ok, 6),
            10
        );

        assert_eq!(
            seg.min_left_recurse(is_ok, 10),
            6
        );
        assert_eq!(
            seg.min_left_recurse(is_ok, 5),
            0
        );
        assert_eq!(
            seg.min_left_recurse(is_ok, 6),
            6
        );
    }
}
