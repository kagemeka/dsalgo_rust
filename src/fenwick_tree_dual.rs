use crate::{
    fenwick_tree,
    group_theory::{AbelianGroup, CommutativeMonoid},
};
pub struct FenwickTreeDual<S, I>
where
    S: CommutativeMonoid<I> + Copy,
    I: crate::group_theory::BinaryOperationIdentifier,
{
    fenwick: fenwick_tree::FenwickTree<S, I>,
}

impl<S, I> FenwickTreeDual<S, I>
where
    S: CommutativeMonoid<I> + Copy,
    I: crate::group_theory::BinaryOperationIdentifier,
{
    pub fn from_deltas(deltas: &[S]) -> Self
    where
        S: Clone,
    {
        Self {
            fenwick: fenwick_tree::FenwickTree::<S, I>::from(deltas),
        }
    }

    pub fn new(size: usize) -> Self
    where
        S: Clone,
    {
        Self::from_deltas(&vec![S::identity(); size].as_slice())
    }

    pub fn size(&self) -> usize { self.fenwick.size() }

    // pub fn set_half_range(&mut self, left: usize,
    // value_to_operate: &S) {
    pub fn set_half_range(&mut self, left: usize, value_to_operate: S) {
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

impl<S, I> From<&[S]> for FenwickTreeDual<S, I>
where
    S: AbelianGroup<I> + Clone + Copy,
    I: crate::group_theory::BinaryOperationIdentifier,
{
    fn from(slice: &[S]) -> Self {
        Self::from_deltas(
            slice
                .iter()
                .enumerate()
                // .map(|(index, prod)| {
                //     S::operate(
                //         &(if index == 0 {
                //             S::identity()
                //         } else {
                //             S::invert(&slice[index - 1])
                //         }),
                //         prod,
                //     )
                // })
                .map(|(index, prod)| {
                    S::operate(
                        if index == 0 {
                            S::identity()
                        } else {
                            slice[index - 1].invert()
                        },
                        *prod,
                    )
                    // S::operate(
                    //     &(if index == 0 {
                    //         S::identity()
                    //     } else {
                    //         S::invert(&slice[index - 1])
                    //     }),
                    //     prod,
                    // )
                })
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }
}

impl<S, I> FenwickTreeDual<S, I>
where
    S: AbelianGroup<I> + Copy,
    I: crate::group_theory::BinaryOperationIdentifier,
{
    // pub fn set_range(&mut self, left: usize, right: usize,
    // value_to_operate: &S) {
    pub fn set_range(
        &mut self,
        left: usize,
        right: usize,
        value_to_operate: S,
    ) {
        assert!(left <= right && right <= self.size());
        if left < self.size() {
            self.set_half_range(left, value_to_operate);
        }
        if right < self.size() {
            // self.set_half_range(right, &S::invert(value_to_operate));
            self.set_half_range(
                right,
                value_to_operate.invert(),
            );
        }
    }

    /// prod[left, index) >= target_value - prod[0, left)
    /// prod[left, index) + prod[0, left) >= target_value
    /// is_ok(G::operate(prod[left, index), prod[0, left)))
    /// `is_ok`'s results must be mnotonous
    /// in the range of [left, self.size())
    /// [?, .., ?, false(left), .., false, true .., true]
    /// where first false index corresponds
    /// to the given left, it might be there exists no
    /// false.
    pub fn binary_search_from_left<F>(&self, is_ok: &F, left: usize) -> usize
    where
        F: Fn(&S) -> bool,
    {
        assert!(left <= self.size());
        let prod_less_than_left = if left == 0 {
            S::identity()
        } else {
            self.get_point(left - 1)
        };
        self.fenwick.find_max_right_with_left(
            // &|prod: &S| !is_ok(&S::operate(&prod_less_than_left, prod)),
            &|prod: &S| {
                !is_ok(&S::operate(
                    prod_less_than_left,
                    *prod,
                ))
            },
            left,
        )
    }

    // /// [false, .. false, true, .., true, ?, .. ?]
    // /// find first true index.
    // /// no longer necessary function.
    // pub fn binary_search_from_right<F>(&self, is_ok: &F, right:
    // usize) -> usize where
    //     F: Fn(&S) -> bool,
    // {
    //     assert!(right <= self.size());
    // }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_as_abelian_group() {
        use crate::group_theory::Additive;
        let deltas = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut fw =
            super::FenwickTreeDual::<i32, Additive>::from_deltas(&deltas);
        assert_eq!(fw.get_point(1), 1);
        assert_eq!(fw.get_point(5), 15);
        assert_eq!(fw.get_point(9), 45);
        // fw.set_half_range(5, &2);
        fw.set_half_range(5, 2);
        assert_eq!(fw.get_point(1), 1);
        assert_eq!(fw.get_point(5), 17);
        assert_eq!(fw.get_point(9), 47);
        assert_eq!(
            fw.binary_search(&|value: &i32| *value >= 23),
            6
        );
        assert_eq!(
            fw.binary_search(&|value: &i32| *value >= 47),
            9
        );
        assert_eq!(
            fw.binary_search(&|value: &i32| *value > 47),
            10
        );

        // abelian group
        let mut arr = deltas;
        arr.iter_mut().fold(0, |acc, x| {
            *x += acc;
            *x
        });
        let mut fw =
            super::FenwickTreeDual::<i32, Additive>::from(arr.as_slice());
        assert_eq!(fw.get_point(1), 1);
        assert_eq!(fw.get_point(5), 15);
        assert_eq!(fw.get_point(9), 45);
        // fw.set_half_range(5, &2);
        fw.set_half_range(5, 2);
        assert_eq!(fw.get_point(1), 1);
        assert_eq!(fw.get_point(5), 17);
        assert_eq!(fw.get_point(9), 47);
        assert_eq!(
            fw.binary_search(&|value: &i32| *value >= 23),
            6
        );
        assert_eq!(
            fw.binary_search(&|value: &i32| *value >= 47),
            9
        );
        assert_eq!(
            fw.binary_search(&|value: &i32| *value > 47),
            10
        );
        // fw.set_range(2, 6, &1);
        fw.set_range(2, 6, 1);
        assert_eq!(fw.get_point(1), 1);
        assert_eq!(fw.get_point(5), 18);
        assert_eq!(fw.get_point(9), 47);
        // fw.set_range(2, 6, &-1);
        fw.set_range(2, 6, -1);
        assert_eq!(
            fw.binary_search_from_left(&|value: &i32| *value >= 23, 0),
            6
        );
        assert_eq!(
            fw.binary_search_from_left(&|value: &i32| *value >= 47, 0),
            9
        );
        assert_eq!(
            fw.binary_search_from_left(&|value: &i32| *value > 47, 0),
            10
        );

        // [0, 1, 3, 6, 10, 17, 23, 30, 38, 47]
        assert_eq!(
            fw.binary_search_from_left(&|value: &i32| *value >= 23, 7),
            7
        );
        assert_eq!(
            fw.binary_search_from_left(&|value: &i32| *value >= 23, 5),
            6
        );
    }
}
