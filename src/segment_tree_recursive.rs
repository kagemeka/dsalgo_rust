
// /// Recursive Implementations for bench mark.
// impl<S, I> SegmentTree<S, I>
// where
//     S: Monoid<I> + Copy,
//     I: crate::group_theory::BinaryOperationIdentifier,
// {
//     pub fn get_range_recurse(&self, left: usize, right: usize) -> S {
//         assert!(left <= right && right <= self.size);
//         self._get_recurse(
//             left,
//             right,
//             0,
//             self.data.len() >> 1,
//             1,
//         )
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
//             // return S::operate(&S::identity(), &self.data[node_index]);
//             return self.data[node_index];
//         }
//         let center = (current_left + current_right) >> 1;
//         // S::operate(
//         //     &self._get_recurse(left, right, current_left, center,
//         // node_index << 1),     &self._get_recurse(left,
//         // right, center, current_right, node_index << 1 | 1),
//         // )
//         self._get_recurse(
//             left,
//             right,
//             current_left,
//             center,
//             node_index << 1,
//         )
//         .operate(self._get_recurse(
//             left,
//             right,
//             center,
//             current_right,
//             node_index << 1 | 1,
//         ))
//     }

//     pub fn find_max_right_recurse<F>(&self, is_ok: &F, left: usize) -> usize
//     where
//         F: Fn(&S) -> bool,
//     {
//         assert!(left <= self.size);
//         self._max_right_recurse(
//             is_ok,
//             left,
//             0,
//             self.data.len() >> 1,
//             &mut S::identity(),
//             1,
//         )
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
//             && is_ok(&current_value.operate(self.data[node_index]))
//         // && is_ok(&S::operate(current_value, &self.data[node_index]))
//         {
//             // *current_value = S::operate(current_value,
//             // &self.data[node_index]);
//             *current_value = current_value.operate(self.data[node_index]);
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
//         self._min_left_recurse(
//             is_ok,
//             right,
//             0,
//             self.data.len() >> 1,
//             &mut S::identity(),
//             1,
//         )
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
//         // if current_right <= right &&
//         // is_ok(&S::operate(&self.data[node_index], current_value)) {
//         if current_right <= right
//             && is_ok(&S::operate(
//                 self.data[node_index],
//                 *current_value,
//             ))
//         {
//             // *current_value = S::operate(&self.data[node_index],
//             // current_value);
//             *current_value = self.data[node_index].operate(*current_value);
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
