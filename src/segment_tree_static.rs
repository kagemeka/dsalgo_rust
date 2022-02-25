use crate::abstract_traits_2::{Additive, Monoid};

pub struct SegmentTree<S: Monoid<S, T>, T = Additive> {
    phantom: std::marker::PhantomData<T>,
    size: usize,
    data: Vec<S>,
}

impl<S: Clone + Monoid<S, T>, T> From<&Vec<S>> for SegmentTree<S, T> {
    fn from(arr: &Vec<S>) -> Self {
        let size = arr.len();
        assert!(size > 0);
        let n = size.next_power_of_two();
        let mut data = vec![S::identity(); n << 1];
        data[n..(n + size)].clone_from_slice(arr);
        let mut seg = Self {
            phantom: std::marker::PhantomData,
            size,
            data,
        };
        for i in (1..n).rev() {
            seg.merge(i);
        }
        seg
    }
}

impl<S: Clone + Monoid<S, T>, T> SegmentTree<S, T> {
    pub fn new(size: usize) -> Self { (&vec![S::identity(); size]).into() }
}

impl<S: Monoid<S, T>, T> SegmentTree<S, T> {
    fn merge(&mut self, i: usize) {
        self.data[i] = S::operate(&self.data[i << 1], &self.data[i << 1 | 1]);
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
        let mut vl = S::identity();
        let mut vr = S::identity();
        while l < r {
            if l & 1 == 1 {
                vl = S::operate(&vl, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = S::operate(&self.data[r], &vr);
            }
            l >>= 1;
            r >>= 1;
        }
        S::operate(&vl, &vr)
    }

    pub fn max_right(
        &self,
        is_ok: Box<dyn Fn(&S) -> bool>,
        left: usize,
    ) -> usize {
        assert!(left < self.size);
        let n = self.data.len() >> 1;
        let mut v = S::identity();
        let mut i = (left + n) as i32;
        loop {
            i /= i & -i;
            if is_ok(&S::operate(&v, &self.data[i as usize])) {
                v = S::operate(&v, &self.data[i as usize]);
                i += 1;
                if i & -i == i {
                    return self.size;
                }
                continue;
            }
            while i < n as i32 {
                i <<= 1;
                if is_ok(&S::operate(&v, &self.data[i as usize])) {
                    v = S::operate(&v, &self.data[i as usize]);
                }
            }
            return i as usize - n;
        }
    }
}

impl<S: Monoid<S, T>, T> std::ops::Index<usize> for SegmentTree<S, T> {
    type Output = S;

    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.size);
        &self.data[i + (self.data.len() >> 1)]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use crate::abstract_traits_2::{Identity, Semigroup};
        // struct Add;

        impl Semigroup for usize {
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
}
