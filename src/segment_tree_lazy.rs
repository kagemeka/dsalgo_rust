use crate::{
    bitwise,
    group_theory::{BinaryOperationIdentifier, Monoid},
    set_theory,
};

pub struct SegmentTreeLazy<S, F, M, I, J>
where
    S: Monoid<I>,
    F: Monoid<J>,
    M: set_theory::Mapping<S, F>,
    I: BinaryOperationIdentifier,
    J: BinaryOperationIdentifier,
{
    phantom_m: std::marker::PhantomData<M>,
    phantom_i: std::marker::PhantomData<I>,
    phantom_j: std::marker::PhantomData<J>,
    size: usize,
    height: u32,
    data: Vec<S>,
    lazy_operators: Vec<F>,
}

impl<S, F, M, I, J> From<&[S]> for SegmentTreeLazy<S, F, M, I, J>
where
    S: Monoid<I> + Clone,
    F: Monoid<J> + Clone,
    M: set_theory::Mapping<S, F>,
    I: BinaryOperationIdentifier,
    J: BinaryOperationIdentifier,
{
    fn from(slice: &[S]) -> Self {
        let size = slice.len();
        let n = size.next_power_of_two();
        let mut data = vec![S::identity(); n << 1];
        data[n..(n + size)].clone_from_slice(slice);
        let mut seg = Self {
            phantom_m: std::marker::PhantomData,
            phantom_i: std::marker::PhantomData,
            phantom_j: std::marker::PhantomData,
            size: slice.len(),
            height: bitwise::bit_length(n),
            data,
            lazy_operators: vec![F::identity(); n],
        };
        for node_index in (1..n).rev() {
            seg.merge_childs(node_index);
        }
        seg
    }
}

impl<S, F, M, I, J> SegmentTreeLazy<S, F, M, I, J>
where
    S: Monoid<I> + Clone,
    F: Monoid<J> + Clone,
    M: set_theory::Mapping<S, F>,
    I: BinaryOperationIdentifier,
    J: BinaryOperationIdentifier,
{
    pub fn new(size: usize) -> Self { (&vec![S::identity(); size]).as_slice().into() }

    pub fn size(&self) -> usize { self.size }

    fn merge_childs(&mut self, node_index: usize) {
        self.data[node_index] =
            S::operate(&self.data[node_index << 1], &self.data[node_index << 1 | 1]);
    }

    fn apply(&mut self, node_index: usize, operator: &F) {
        self.data[node_index] = M::map(operator, &self.data[node_index]);
        if node_index < self.lazy_operators.len() {
            self.lazy_operators[node_index] =
                F::operate(operator, &self.lazy_operators[node_index]);
        }
    }

    fn propagate(&mut self, node_index: usize) {
        let operator = &self.lazy_operators[node_index].clone();
        self.apply(node_index << 1, operator);
        self.apply(node_index << 1 | 1, operator);
        self.lazy_operators[node_index] = F::identity();
    }

    fn propagate_above(&mut self, node_index: usize) {
        for i in (1..self.height).rev() {
            // if (node_index >> i) << i != node_index {
            //     self.propagate(node_index >> i);
            // }
            self.propagate(node_index >> i);
        }
    }

    fn update_above(&mut self, node_index: usize) {
        for i in 1..self.height {
            // if (node_index >> i) << i != node_index {
            //     self.merge_childs(node_index >> i);
            // }
            self.merge_childs(node_index >> i);
        }
    }

    pub fn set_range(&mut self, left: usize, right: usize, operator: &F) {
        assert!(left <= right && right <= self.size);
        let n = self.data.len() >> 1;
        let mut left_node = n + left;
        let mut right_node = n + right;

        self.propagate_above(left_node);
        self.propagate_above(right_node);

        let (left_0, right_0) = (left_node, right_node);
        // operate on target range.
        while left_node < right_node {
            if left_node & 1 == 1 {
                self.apply(left_node, operator);
                left_node += 1;
            }
            if right_node & 1 == 1 {
                right_node -= 1;
                self.apply(right_node, operator);
            }
            left_node >>= 1;
            right_node >>= 1;
        }
        self.update_above(left_0);
        self.update_above(right_0);
    }

    pub fn update_point(&mut self, array_index: usize, value: S) {
        assert!(array_index < self.size);
        let n = self.data.len() >> 1;
        let node_index = n + array_index;
        self.propagate_above(node_index);
        self.data[node_index] = value;
        self.update_above(node_index);
    }

    pub fn get_range(&mut self, left: usize, right: usize) -> S {
        assert!(left <= right && right <= self.size);
        let n = self.data.len() >> 1;
        let mut left_node = n + left;
        let mut right_node = n + right;

        self.propagate_above(left_node);
        self.propagate_above(right_node);

        let mut left_value = S::identity();
        let mut right_value = S::identity();
        while left_node < right_node {
            if left_node & 1 == 1 {
                left_value = S::operate(&left_value, &self.data[left_node]);
                left_node += 1;
            }
            if right_node & 1 == 1 {
                right_node -= 1;
                right_value = S::operate(&self.data[right_node], &right_value);
            }
            left_node >>= 1;
            right_node >>= 1;
        }
        S::operate(&left_value, &right_value)
    }

    pub fn get_point(&mut self, array_index: usize) -> S {
        assert!(array_index < self.size);
        self.get_range(array_index, array_index + 1)
    }

    // pub fn find_max_right<G>(&self, is_ok: &F, left: usize) ->
    // usize where
    //     G: Fn(&S) -> bool,
    // {
    //     assert!(left <= self.size);
    //     if left == self.size {
    //         return self.size;
    //     }
    //     let n = self.data.len() >> 1;
    //     let mut value = S::identity();
    //     let mut node_index = n + left;
    //     loop {
    //         node_index =
    // bitwise::shift_right_until_odd(node_index).unwrap(); // up
    // to ceil         if !is_ok(&S::operate(&value,
    // &self.data[node_index])) {             break;
    //         }
    //         // up one stair from left
    //         value = S::operate(&value, &self.data[node_index]);
    //         node_index += 1;
    //         if bitwise::lsb_number(node_index) == node_index {
    //             // wall.
    //             return self.size;
    //         }
    //     }
    //     // down stairs to right
    //     while node_index < n {
    //         node_index <<= 1;
    //         if !is_ok(&S::operate(&value,
    // &self.data[node_index])) {             continue;
    //         }
    //         value = S::operate(&value, &self.data[node_index]);
    //         node_index += 1;
    //     }
    //     node_index - n
    // }

    // pub fn find_min_left<F>(&self, is_ok: &F, right: usize) ->
    // usize where
    //     F: Fn(&S) -> bool,
    // {
    //     assert!(right <= self.size);
    //     if right == 0 {
    //         return 0;
    //     }
    //     let n = self.data.len() >> 1;
    //     let mut value = S::identity();
    //     let mut node_index = n + right;
    //     loop {
    //         assert!(node_index >= 1);
    //         node_index =
    // bitwise::shift_right_until_odd(node_index).unwrap();
    //         assert!(node_index >= 1);
    //         if !is_ok(&S::operate(&self.data[node_index - 1],
    // &value)) {             break;
    //         }
    //         node_index -= 1;
    //         value = S::operate(&self.data[node_index as usize],
    // &value);         if bitwise::lsb_number(node_index) ==
    // node_index {             return 0;
    //         }
    //     }
    //     while node_index < n {
    //         node_index <<= 1;
    //         if !is_ok(&S::operate(&self.data[node_index - 1],
    // &value)) {             continue;
    //         }
    //         node_index -= 1;
    //         value = S::operate(&self.data[node_index], &value);
    //     }
    //     node_index - n
    // }
}

/// Recursive Implementations for bench mark.
// impl<S: Monoid<T>, T> SegmentTree<S, T>
// where
//     T: crate::group_theory::BinaryOperationIdentifier,
// {
//     pub fn get_range_recurse(&self, left: usize, right: usize) -> S {
//         assert!(left <= right && right <= self.size);
//         self._get_recurse(left, right, 0, self.data.len() >> 1, 1)
//     }

//     fn _get_recurse(
//         &self,
//         left: usize,
//         right: usize,
//         current_left: usize,
//         current_right: usize,
//         node_index: usize,
//     ) -> S {
//         if current_right <= left || right <= current_left {
//             return S::identity();
//         }
//         if left <= current_left && current_right <= right {
//             return S::operate(&S::identity(), &self.data[node_index]);
//         }
//         let center = (current_left + current_right) >> 1;
//         S::operate(
//             &self._get_recurse(left, right, current_left, center, node_index << 1),
//             &self._get_recurse(left, right, center, current_right, node_index << 1 | 1),
//         )
//     }

//     pub fn find_max_right_recurse<F>(&self, is_ok: &F, left: usize) -> usize
//     where
//         F: Fn(&S) -> bool,
//     {
//         assert!(left <= self.size);
//         self._max_right_recurse(is_ok, left, 0, self.data.len() >> 1, &mut S::identity(), 1)
//     }

//     /// find max right (current_left < right <= current_right)
//     /// if current_right <= left, return left
//     /// if current_left >= self.size, return self.size
//     fn _max_right_recurse<F>(
//         &self,
//         is_ok: &F,
//         left: usize,
//         current_left: usize,
//         current_right: usize,
//         current_value: &mut S,
//         node_index: usize,
//     ) -> usize
//     where
//         F: Fn(&S) -> bool,
//     {
//         if current_right <= left {
//             return left;
//         }
//         if current_left >= self.size {
//             return self.size;
//         }
//         if left <= current_left
//             && current_right <= self.size
//             && is_ok(&S::operate(current_value, &self.data[node_index]))
//         {
//             *current_value = S::operate(current_value, &self.data[node_index]);
//             return current_right;
//         }
//         if current_right - current_left == 1 {
//             return current_left;
//         }
//         let center = (current_left + current_right) >> 1;
//         let right = self._max_right_recurse(
//             is_ok,
//             left,
//             current_left,
//             center,
//             current_value,
//             node_index << 1,
//         );
//         if right < center || right == self.size {
//             return right;
//         }
//         self._max_right_recurse(
//             is_ok,
//             left,
//             center,
//             current_right,
//             current_value,
//             node_index << 1 | 1,
//         )
//     }

//     pub fn find_min_left_recurse<F>(&self, is_ok: &F, right: usize) -> usize
//     where
//         F: Fn(&S) -> bool,
//     {
//         assert!(right <= self.size);
//         self._min_left_recurse(is_ok, right, 0, self.data.len() >> 1, &mut S::identity(), 1)
//     }

//     fn _min_left_recurse<F>(
//         &self,
//         is_ok: &F,
//         right: usize,
//         current_left: usize,
//         current_right: usize,
//         current_value: &mut S,
//         node_index: usize,
//     ) -> usize
//     where
//         F: Fn(&S) -> bool,
//     {
//         if current_left >= right {
//             return right;
//         }
//         if current_right <= right && is_ok(&S::operate(&self.data[node_index], current_value)) {
//             *current_value = S::operate(&self.data[node_index], current_value);
//             return current_left;
//         }
//         if current_right - current_left == 1 {
//             return current_right;
//         }
//         let center = (current_left + current_right) >> 1;
//         let left = self._min_left_recurse(
//             is_ok,
//             right,
//             center,
//             current_right,
//             current_value,
//             node_index << 1 | 1,
//         );
//         if left > center || left == 0 {
//             return left;
//         }
//         self._min_left_recurse(
//             is_ok,
//             right,
//             current_left,
//             center,
//             current_value,
//             node_index << 1,
//         )
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

// class LazySegmentTreeDFS(LazySegmentTree[S, F]):
//     def set(self, left: int, right: int, f: F) -> None:
//         assert 0 <= left <= right <= self.size
//         self.__set(left, right, f, 0, len(self) >> 1, 1)

//     def __set(
//         self,
//         left: int,
//         right: int,
//         f: F,
//         current_left: int,
//         current_right: int,
//         i: int,
//     ) -> None:
//         n = len(self) >> 1
//         if i < n:
//             self._propagate(i)
//         if current_right <= left or right <= current_left:
//             return
//         if left <= current_left and current_right <= right:
//             self._apply(i, f)
//             if i < n:
//                 self._propagate(i)
//             return
//         center = (current_left + current_right) >> 1
//         self.__set(left, right, f, current_left, center, i
// << 1)         self.__set(left, right, f, center,
// current_right, i << 1 | 1)         self._merge(i)

//     def get(self, left: int, right: int) -> S:
//         assert 0 <= left <= right <= self.size
//         return self.__get(left, right, 0, len(self) >> 1, 1)

//     def __get(
//         self,
//         left: int,
//         right: int,
//         current_left: int,
//         current_right: int,
//         i: int,
//     ) -> S:
//         n = len(self) >> 1
//         if i < n:
//             self._propagate(i)
//         if current_right <= left or right <= current_left:
//             return self._monoid_s.identity()
//         if left <= current_left and current_right <= right:
//             if i < n:
//                 self._propagate(i)
//             return self._data[i]
//         center = (current_left + current_right) >> 1
//         vl = self.__get(left, right, current_left, center, i
// << 1)         vr = self.__get(left, right, center,
// current_right, i << 1 | 1)         self._merge(i)
//         return self._monoid_s.operation(vl, vr)

//     def update(self, i: int, x: S) -> None:
//         assert 0 <= i < self.size
//         n = len(self) >> 1
//         self.get(i, i + 1)
//         self._data[n + i] = x
//         self.get(i, i + 1)
