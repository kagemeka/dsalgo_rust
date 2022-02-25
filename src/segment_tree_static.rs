use crate::abstract_traits::{Additive, Monoid};

pub struct SegmentTree<M: Monoid<S, T>, S = M, T = Additive> {
    phantom: std::marker::PhantomData<T>,
    phantom_m: std::marker::PhantomData<M>,
    size: usize,
    data: Vec<S>,
}

impl<M: Monoid<S, T>, S: Clone, T> From<&Vec<S>> for SegmentTree<M, S, T> {
    fn from(arr: &Vec<S>) -> Self {
        let size = arr.len();
        assert!(size > 0);
        let n = size.next_power_of_two();
        let mut data = vec![M::identity(); n << 1];
        data[n..(n + size)].clone_from_slice(arr);
        let mut seg = Self {
            phantom: std::marker::PhantomData,
            phantom_m: std::marker::PhantomData,
            size,
            data,
        };
        for i in (1..n).rev() {
            seg.merge(i);
        }
        seg
    }
}

impl<M: Monoid<S, T>, S: Clone, T> SegmentTree<M, S, T> {
    pub fn new(size: usize) -> Self { (&vec![M::identity(); size]).into() }
}

impl<M: Monoid<S, T>, S, T> SegmentTree<M, S, T> {
    fn merge(&mut self, i: usize) {
        self.data[i] = M::operate(&self.data[i << 1], &self.data[i << 1 | 1]);
    }

    pub fn set(&mut self, mut i: usize, x: S) {
        assert!(i < self.size);
        i += self.data.len() >> 1;
        self.data[i] = x;
        while i > 1 {
            i >>= 1;
            self.merge(i);
        }
    }

    pub fn get(&self, mut l: usize, mut r: usize) -> S {
        assert!(l <= r && r <= self.size);
        let n = self.data.len() >> 1;
        l += n;
        r += n;
        let mut vl = M::identity();
        let mut vr = M::identity();
        while l < r {
            if l & 1 == 1 {
                vl = M::operate(&vl, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = M::operate(&self.data[r], &vr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::operate(&vl, &vr)
    }

    pub fn max_right(
        &self,
        is_ok: Box<dyn Fn(&S) -> bool>,
        left: usize,
    ) -> usize {
        assert!(left < self.size);
        let n = self.data.len() >> 1;
        let mut v = M::identity();
        let mut i = (left + n) as i32;
        loop {
            i /= i & -i;
            if is_ok(&M::operate(&v, &self.data[i as usize])) {
                v = M::operate(&v, &self.data[i as usize]);
                i += 1;
                if i & -i == i {
                    return self.size;
                }
                continue;
            }
            while i < n as i32 {
                i <<= 1;
                if is_ok(&M::operate(&v, &self.data[i as usize])) {
                    v = M::operate(&v, &self.data[i as usize]);
                }
            }
            return i as usize - n;
        }
    }
}

impl<M: Monoid<S, T>, S, T> std::ops::Index<usize> for SegmentTree<M, S, T> {
    type Output = S;

    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.size);
        &self.data[i + (self.data.len() >> 1)]
    }
}

#[cfg(test)]
mod tests {
    use crate::abstract_traits::{BinaryOperation, Identity};
    #[test]
    fn test_as_monoid() {
        impl BinaryOperation for usize {
            fn operate(x: &Self, y: &Self) -> Self { x + y }
        }
        impl Identity for usize {
            fn identity() -> Self { 0 }
        }

        let mut seg = super::SegmentTree::<usize>::new(10);
        assert_eq!(seg.get(0, 10), 0);
        seg.set(0, 5);
        assert_eq!(seg.get(0, 10), 5);
        seg.set(0, 5);
        assert_eq!(seg[0], 5);
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
        assert_eq!(seg.get(0, 10), 0);
        seg.set(0, 5);
        assert_eq!(seg.get(0, 10), 5);
        seg.set(0, 5);
        assert_eq!(seg[0], 5);
    }
}
