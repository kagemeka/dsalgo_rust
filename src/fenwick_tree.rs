// use crate::algebra::abstract_::structure::structs::Monoid;
use crate::abstraction;
pub struct FenwickTree<'a, S: Copy> {
    m: abstraction::structure::structs::Monoid<'a, S>,
    data: Vec<S>,
}

impl<'a, S: std::fmt::Debug + Copy> std::fmt::Debug for FenwickTree<'a, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FenwickTree").field(&self.data).finish()
    }
}

impl<'a, S: Copy> FenwickTree<'a, S> {
    pub fn new(
        m: abstraction::structure::structs::Monoid<'a, S>,
        n: usize,
    ) -> Self {
        let a = vec![(m.e)(); n];
        Self::from_vec(m, &a)
    }

    pub fn from_vec(
        m: abstraction::structure::structs::Monoid<'a, S>,
        a: &Vec<S>,
    ) -> Self {
        let n = a.len();
        let mut data = vec![(m.e)(); n + 1];
        for i in 0..n {
            data[i + 1] = a[i];
        }
        for i in 1..=n as i32 {
            let j = (i + (i & -i)) as usize;
            if j < n + 1 {
                data[j] = (m.op)(&data[j], &data[i as usize]);
            }
        }
        Self { m, data }
    }

    pub fn set(&mut self, mut i: usize, x: &S) {
        assert!(i < self.data.len() - 1);
        i += 1;
        while i < self.data.len() {
            self.data[i] = (self.m.op)(&self.data[i], x);
            i += (i as i32 & -(i as i32)) as usize;
        }
    }

    pub fn get(&self, mut i: usize) -> S {
        assert!(i < self.data.len());
        let mut v = (self.m.e)();
        while i > 0 {
            v = (self.m.op)(&v, &self.data[i]);
            i -= (i as i32 & -(i as i32)) as usize;
        }
        v
    }

    pub fn max_right(&self, is_ok: Box<dyn Fn(&S) -> bool>) -> usize {
        let n = self.data.len();
        let mut l = 1;
        while l << 1 < n {
            l <<= 1;
        }
        let mut v = (self.m.e)();
        let mut i = 0usize;
        while l > 0 {
            if i + l < n && is_ok(&(self.m.op)(&v, &self.data[i + l])) {
                i += l;
                v = (self.m.op)(&v, &self.data[i + 1]);
            }
            l >>= 1;
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenwick_tree() {
        let op = |x: &i32, y: &i32| x + y;
        let e = || 0;
        let m = abstraction::structure::structs::Monoid::<i32> {
            op: &op,
            e: &e,
            commutative: true,
            idempotent: false,
        };
        let mut a = vec![0i32; 10];
        for i in 0..10i32 {
            a[i as usize] = i;
        }
        let mut fw = FenwickTree::from_vec(m, &a);
        assert_eq!(fw.get(10), 45);
        assert_eq!(
            fw.data,
            vec![
                0, 0, 1, 2, 6, 4, 9, 6, 28, 8, 17
            ],
        );
        println!("{:?}", fw);
        fw.set(4, &4);
        assert_eq!(fw.get(8) - fw.get(3), 29);
    }
}
