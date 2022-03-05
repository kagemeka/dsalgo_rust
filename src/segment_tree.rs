use crate::{
    bitwise,
    group_theory::{Additive, Monoid},
};

/// Node Indices (case $4 \lt |given array| \le 8$)
/// |1                      |2
/// |2          |3          |4
/// |4    |5    |6    |7    |8
/// |8 |9 |10|11|12|13|14|15|16
pub struct SegmentTree<M: Monoid<S, T>, S = M, T = Additive> {
    phantom_t: std::marker::PhantomData<T>,
    phantom_m: std::marker::PhantomData<M>,
    size: usize,
    data: Vec<S>,
}

impl<M: Monoid<S, T>, S: Clone, T> From<&[S]> for SegmentTree<M, S, T> {
    fn from(slice: &[S]) -> Self {
        let size = slice.len();
        let n = size.next_power_of_two();
        let mut data = vec![M::identity(); n << 1];
        data[n..(n + size)].clone_from_slice(slice);
        let mut seg = Self {
            phantom_t: std::marker::PhantomData,
            phantom_m: std::marker::PhantomData,
            size: slice.len(),
            data,
        };
        for node_index in (1..n).rev() {
            seg.merge_childs(node_index);
        }
        seg
    }
}

impl<M: Monoid<S, T>, S, T> SegmentTree<M, S, T> {
    pub fn new(size: usize) -> Self
    where
        S: Clone,
    {
        (&vec![M::identity(); size]).as_slice().into()
    }

    pub fn size(&self) -> usize { self.size }

    fn merge_childs(&mut self, node_index: usize) {
        self.data[node_index] =
            M::operate(&self.data[node_index << 1], &self.data[node_index << 1 | 1]);
    }

    pub fn set_point(&mut self, array_index: usize, x: S) {
        assert!(array_index < self.size);
        let mut node_index = array_index + (self.data.len() >> 1);
        self.data[node_index] = x;
        while node_index > 1 {
            node_index >>= 1;
            self.merge_childs(node_index);
        }
    }

    pub fn get_range(&self, left: usize, right: usize) -> S {
        assert!(left <= right && right <= self.size);
        let n = self.data.len() >> 1;
        let mut left_node_index = n + left;
        let mut right_node_index = n + right;
        let mut value_left = M::identity();
        let mut value_right = M::identity();
        while left_node_index < right_node_index {
            if left_node_index & 1 == 1 {
                value_left = M::operate(&value_left, &self.data[left_node_index]);
                left_node_index += 1;
            }
            if right_node_index & 1 == 1 {
                right_node_index -= 1;
                value_right = M::operate(&self.data[right_node_index], &value_right);
            }
            left_node_index >>= 1;
            right_node_index >>= 1;
        }
        M::operate(&value_left, &value_right)
    }

    pub fn find_max_right<F>(&self, is_ok: &F, left: usize) -> usize
    where
        F: Fn(&S) -> bool,
    {
        assert!(left <= self.size);
        if left == self.size {
            return self.size;
        }
        let n = self.data.len() >> 1;
        let mut value = M::identity();
        let mut node_index = n + left;
        loop {
            node_index = bitwise::shift_right_until_odd(node_index).unwrap(); // up to ceil
            if !is_ok(&M::operate(&value, &self.data[node_index])) {
                break;
            }
            // up one stair from left
            value = M::operate(&value, &self.data[node_index]);
            node_index += 1;
            if bitwise::lsb_number(node_index) == node_index {
                // wall.
                return self.size;
            }
        }
        // down stairs to right
        while node_index < n {
            node_index <<= 1;
            if !is_ok(&M::operate(&value, &self.data[node_index])) {
                continue;
            }
            value = M::operate(&value, &self.data[node_index]);
            node_index += 1;
        }
        node_index - n
    }

    pub fn find_min_left<F>(&self, is_ok: &F, right: usize) -> usize
    where
        F: Fn(&S) -> bool,
    {
        assert!(right <= self.size);
        if right == 0 {
            return 0;
        }
        let n = self.data.len() >> 1;
        let mut value = M::identity();
        let mut node_index = n + right;
        loop {
            assert!(node_index >= 1);
            node_index = bitwise::shift_right_until_odd(node_index).unwrap();
            assert!(node_index >= 1);
            if !is_ok(&M::operate(&self.data[node_index - 1], &value)) {
                break;
            }
            node_index -= 1;
            value = M::operate(&self.data[node_index as usize], &value);
            if bitwise::lsb_number(node_index) == node_index {
                return 0;
            }
        }
        while node_index < n {
            node_index <<= 1;
            if !is_ok(&M::operate(&self.data[node_index - 1], &value)) {
                continue;
            }
            node_index -= 1;
            value = M::operate(&self.data[node_index], &value);
        }
        node_index - n
    }
}

impl<M: Monoid<S, T>, S, T> std::ops::Index<usize> for SegmentTree<M, S, T> {
    type Output = S;

    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.size);
        &self.data[i + (self.data.len() >> 1)]
    }
}

/// Recursive Implementations for bench mark.
impl<M: Monoid<S, T>, S, T> SegmentTree<M, S, T> {
    pub fn get_range_recurse(&self, left: usize, right: usize) -> S {
        assert!(left <= right && right <= self.size);
        self._get_recurse(left, right, 0, self.data.len() >> 1, 1)
    }

    fn _get_recurse(
        &self,
        left: usize,
        right: usize,
        current_left: usize,
        current_right: usize,
        node_index: usize,
    ) -> S {
        if current_right <= left || right <= current_left {
            return M::identity();
        }
        if left <= current_left && current_right <= right {
            return M::operate(&M::identity(), &self.data[node_index]);
        }
        let center = (current_left + current_right) >> 1;
        M::operate(
            &self._get_recurse(left, right, current_left, center, node_index << 1),
            &self._get_recurse(left, right, center, current_right, node_index << 1 | 1),
        )
    }

    pub fn find_max_right_recurse<F>(&self, is_ok: &F, left: usize) -> usize
    where
        F: Fn(&S) -> bool,
    {
        assert!(left <= self.size);
        self._max_right_recurse(is_ok, left, 0, self.data.len() >> 1, &mut M::identity(), 1)
    }

    /// find max right (current_left < right <= current_right)
    /// if current_right <= left, return left
    /// if current_left >= self.size, return self.size
    fn _max_right_recurse<F>(
        &self,
        is_ok: &F,
        left: usize,
        current_left: usize,
        current_right: usize,
        current_value: &mut S,
        node_index: usize,
    ) -> usize
    where
        F: Fn(&S) -> bool,
    {
        if current_right <= left {
            return left;
        }
        if current_left >= self.size {
            return self.size;
        }
        if left <= current_left
            && current_right <= self.size
            && is_ok(&M::operate(current_value, &self.data[node_index]))
        {
            *current_value = M::operate(current_value, &self.data[node_index]);
            return current_right;
        }
        if current_right - current_left == 1 {
            return current_left;
        }
        let center = (current_left + current_right) >> 1;
        let right = self._max_right_recurse(
            is_ok,
            left,
            current_left,
            center,
            current_value,
            node_index << 1,
        );
        if right < center || right == self.size {
            return right;
        }
        self._max_right_recurse(
            is_ok,
            left,
            center,
            current_right,
            current_value,
            node_index << 1 | 1,
        )
    }

    pub fn find_min_left_recurse<F>(&self, is_ok: &F, right: usize) -> usize
    where
        F: Fn(&S) -> bool,
    {
        assert!(right <= self.size);
        self._min_left_recurse(is_ok, right, 0, self.data.len() >> 1, &mut M::identity(), 1)
    }

    fn _min_left_recurse<F>(
        &self,
        is_ok: &F,
        right: usize,
        current_left: usize,
        current_right: usize,
        current_value: &mut S,
        node_index: usize,
    ) -> usize
    where
        F: Fn(&S) -> bool,
    {
        if current_left >= right {
            return right;
        }
        if current_right <= right && is_ok(&M::operate(&self.data[node_index], current_value)) {
            *current_value = M::operate(&self.data[node_index], current_value);
            return current_left;
        }
        if current_right - current_left == 1 {
            return current_right;
        }
        let center = (current_left + current_right) >> 1;
        let left = self._min_left_recurse(
            is_ok,
            right,
            center,
            current_right,
            current_value,
            node_index << 1 | 1,
        );
        if left > center || left == 0 {
            return left;
        }
        self._min_left_recurse(
            is_ok,
            right,
            current_left,
            center,
            current_value,
            node_index << 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::group_theory::{BinaryOperation, Identity};
    #[test]
    fn test_as_monoid() {
        impl BinaryOperation for usize {
            fn operate(x: &Self, y: &Self) -> Self { x + y }
        }
        impl Identity for usize {
            fn identity() -> Self { 0 }
        }

        let mut seg = super::SegmentTree::<usize>::new(10);
        assert_eq!(seg.get_range(0, 10), 0);
        assert_eq!(seg.get_range_recurse(0, 10), 0);
        seg.set_point(5, 5);
        assert_eq!(seg.get_range(0, 10), 5);
        assert_eq!(seg.get_range_recurse(0, 10), 5);
        seg.set_point(5, 10);
        assert_eq!(seg[5], 10);
        assert_eq!(seg[0], 0);
        let is_ok = &|sum: &usize| *sum < 10;
        assert_eq!(seg.find_max_right(is_ok, 0), 5);
        assert_eq!(seg.find_max_right_recurse(is_ok, 0), 5);
        assert_eq!(seg.find_max_right(is_ok, 10), 10);
        assert_eq!(seg.find_max_right_recurse(is_ok, 10), 10);
        assert_eq!(seg.find_max_right(is_ok, 5), 5);
        assert_eq!(seg.find_max_right_recurse(is_ok, 5), 5);
        assert_eq!(seg.find_min_left(is_ok, 10), 6);
        assert_eq!(seg.find_min_left_recurse(is_ok, 10), 6);
        assert_eq!(seg.find_min_left(is_ok, 5), 0);
        assert_eq!(seg.find_min_left_recurse(is_ok, 5), 0);
        assert_eq!(seg.find_min_left(is_ok, 6), 6);
        assert_eq!(seg.find_min_left_recurse(is_ok, 6), 6);

        seg = super::SegmentTree::<usize>::new(0);
        assert_eq!(seg.get_range(0, 0), 0);
    }

    #[test]
    fn test_wrapping_monoid() {
        struct UsizeAdd;

        impl BinaryOperation<usize> for UsizeAdd {
            fn operate(x: &usize, y: &usize) -> usize { x + y }
        }
        impl Identity<usize> for UsizeAdd {
            fn identity() -> usize { 0 }
        }

        let mut seg = super::SegmentTree::<UsizeAdd, usize>::new(10);
        assert_eq!(seg.get_range(0, 10), 0);
        seg.set_point(0, 5);
        assert_eq!(seg.get_range(0, 10), 5);
        seg.set_point(0, 5);
        assert_eq!(seg[0], 5);
    }
}
