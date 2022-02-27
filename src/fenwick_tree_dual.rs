use crate::{
    abstract_traits::{AbelianGroup, Additive, Commutative, Monoid},
    fenwick_tree,
};
pub struct FenwickTreeDual<M: Monoid<S, T> + Commutative<S, T>, S = M, T = Additive> {
    fenwick: fenwick_tree::FenwickTree<M, S, T>,
}

impl<M: Monoid<S, T> + Commutative<S, T>, S, T> FenwickTreeDual<M, S, T> {
    pub fn from_deltas(deltas: &[S]) -> Self
    where
        S: Clone,
    {
        Self {
            fenwick: fenwick_tree::FenwickTree::<M, S, T>::from(deltas),
        }
    }

    pub fn new(size: usize) -> Self
    where
        S: Clone,
    {
        Self::from_deltas(&vec![M::identity(); size].as_slice())
    }

    pub fn size(&self) -> usize { self.fenwick.size() }

    pub fn set_half_range(&mut self, left: usize, value_to_operate: &S) {
        assert!(left <= self.size());
        if left < self.size() {
            self.fenwick.set_point(left, value_to_operate)
        }
    }

    pub fn get_point(&self, array_index: usize) -> S {
        assert!(array_index < self.size());
        self.fenwick.get_half_range(array_index + 1)
    }

    /// find first index i satisfying
    /// `is_ok(&self.get_point(i)) == true`
    /// Constraints:
    /// `is_ok(&self.get_point(i))` must be monotonous [false,
    /// false, .., true, true] if such an i is not found,
    /// return `self.size()`
    pub fn binary_search<F>(&self, is_ok: &F) -> usize
    where
        F: Fn(&S) -> bool,
    {
        self.fenwick.find_max_right(&|prod: &S| !is_ok(prod))
    }
}

impl<G: AbelianGroup<S, T>, S: Clone, T> From<&[S]> for FenwickTreeDual<G, S, T> {
    fn from(slice: &[S]) -> Self {
        Self::from_deltas(
            slice
                .iter()
                .enumerate()
                .map(|(index, prod)| {
                    G::operate(
                        &(if index == 0 {
                            G::identity()
                        } else {
                            G::invert(&slice[index - 1])
                        }),
                        prod,
                    )
                })
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }
}

impl<G: AbelianGroup<S, T>, S, T> FenwickTreeDual<G, S, T> {
    pub fn set_range(&mut self, left: usize, right: usize, value_to_operate: &S) {
        assert!(left <= right && right <= self.size());
        if left < self.size() {
            self.set_half_range(left, value_to_operate);
        }
        if right < self.size() {
            self.set_half_range(right, &G::invert(value_to_operate));
        }
    }

    //     pub fn find_max_right_with_left<F>(&self, is_ok: &F,
    // left: usize) -> usize     where
    //         F: Fn(&S) -> bool,
    //     {
    //         assert!(left <= self.size());
    //         if left == self.size() {
    //             return self.size();
    //         }
    //         let mut length = 1usize <<
    // bitwise::most_significant_bit(self.size()).unwrap();
    //         let mut value =
    // G::invert(&self.get_half_range(left));         let mut right
    // = 0;         while length > 0 {
    //             if right + length <= left
    //                 || right + length <= self.size()
    //                     && is_ok(&G::operate(&value,
    // &self.data[right + length]))             {
    //                 right += length;
    //                 value = G::operate(&value,
    // &self.data[right]);             }
    //             length >>= 1;
    //         }
    //         right
    //     }

    //     pub fn find_min_left_with_right<F>(&self, is_ok: &F,
    // right: usize) -> usize     where
    //         F: Fn(&S) -> bool,
    //     {
    //         assert!(right <= self.size());
    //         if right == 0 {
    //             return 0;
    //         }
    //         let mut length = 1usize <<
    // bitwise::most_significant_bit(self.size()).unwrap();
    //         let mut value = self.get_half_range(right);
    //         if is_ok(&value) {
    //             return 0;
    //         }
    //         let mut left = 1;
    //         while length > 0 {
    //             if left + length <= right
    //                 &&
    // !is_ok(&G::operate(&G::invert(&self.data[left - 1 +
    // length]), &value))             {
    //                 left += length;
    //                 value =
    // G::operate(&G::invert(&self.data[left - 1]), &value);
    //             }
    //             length >>= 1;
    //         }
    //         left
    //     }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_as_abelian_group() {
        use crate::abstract_traits::{BinaryOperation, Commutative, Identity, Inverse};

        struct Add;
        impl Identity<Self, Add> for i32 {
            fn identity() -> Self { 0 }
        }
        impl BinaryOperation<Self, Add> for i32 {
            fn operate(x: &Self, y: &Self) -> Self { x + y }
        }
        impl Commutative<Self, Add> for i32 {}
        impl Inverse<Self, Add> for i32 {
            fn invert(value: &Self) -> i32 { -value }
        }

        let deltas = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut fw = super::FenwickTreeDual::<i32, i32, Add>::from_deltas(&deltas);
        assert_eq!(fw.get_point(1), 1);
        assert_eq!(fw.get_point(5), 15);
        assert_eq!(fw.get_point(9), 45);
        fw.set_half_range(5, &2);
        assert_eq!(fw.get_point(1), 1);
        assert_eq!(fw.get_point(5), 17);
        assert_eq!(fw.get_point(9), 47);
        assert_eq!(fw.binary_search(&|value: &i32| *value >= 23), 6);
        assert_eq!(fw.binary_search(&|value: &i32| *value >= 47), 9);
        assert_eq!(fw.binary_search(&|value: &i32| *value > 47), 10);

        // abelian group
        let mut arr = deltas;
        arr.iter_mut().fold(0, |acc, x| {
            *x += acc;
            *x
        });
        let mut fw = super::FenwickTreeDual::<i32, i32, Add>::from(arr.as_slice());
        assert_eq!(fw.get_point(1), 1);
        assert_eq!(fw.get_point(5), 15);
        assert_eq!(fw.get_point(9), 45);
        fw.set_half_range(5, &2);
        assert_eq!(fw.get_point(1), 1);
        assert_eq!(fw.get_point(5), 17);
        assert_eq!(fw.get_point(9), 47);
        assert_eq!(fw.binary_search(&|value: &i32| *value >= 23), 6);
        assert_eq!(fw.binary_search(&|value: &i32| *value >= 47), 9);
        assert_eq!(fw.binary_search(&|value: &i32| *value > 47), 10);
        fw.set_range(2, 6, &1);
        assert_eq!(fw.get_point(1), 1);
        assert_eq!(fw.get_point(5), 18);
        assert_eq!(fw.get_point(9), 47);
    }
}
