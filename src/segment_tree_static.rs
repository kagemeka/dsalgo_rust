use crate::abstract_traits;

pub trait Monoid {
    type S: Clone;

    fn operate(x: &Self::S, y: &Self::S) -> Self::S;
    fn identity() -> Self::S;
}

pub struct SegmentTree<M: Monoid> {
    size: usize,
    data: Vec<M::S>,
}

impl<M: Monoid> From<&Vec<M::S>> for SegmentTree<M> {
    fn from(arr: &Vec<M::S>) -> Self {
        let size = arr.len();
        assert!(size > 0);
        let n = size.next_power_of_two();
        let mut data = vec![M::identity(); n << 1];
        data[n..(n + size)].clone_from_slice(arr);
        let mut seg = Self { size, data };
        for i in (1..n).rev() {
            seg.merge(i);
        }
        seg
    }
}

impl<M: Monoid> SegmentTree<M> {
    pub fn new(size: usize) -> Self { (&vec![M::identity(); size]).into() }

    fn merge(&mut self, i: usize) {
        self.data[i] = M::operate(&self.data[i << 1], &self.data[i << 1 | 1]);
    }

    pub fn set(&mut self, mut i: usize, x: M::S) {
        assert!(i < self.size);
        i += self.data.len() >> 1;
        self.data[i] = x;
        while i > 1 {
            i >>= 1;
            self.merge(i);
        }
    }

    pub fn get(&self, mut l: usize, mut r: usize) -> M::S {
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
        is_ok: Box<dyn Fn(&M::S) -> bool>,
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

impl<M: Monoid> std::ops::Index<usize> for SegmentTree<M> {
    type Output = M::S;

    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.size);
        &self.data[i + (self.data.len() >> 1)]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        impl super::Monoid for usize {
            type S = usize;

            fn operate(x: &Self::S, y: &Self::S) -> Self::S { x + y }

            fn identity() -> Self::S { 0 }
        }

        let mut seg = super::SegmentTree::<usize>::new(10);
        assert_eq!(seg.get(0, 10), 0);
        seg.set(0, 5);
        assert_eq!(seg.get(0, 10), 5);
        seg.set(0, 5);
        assert_eq!(seg[0], 5);
    }
}
