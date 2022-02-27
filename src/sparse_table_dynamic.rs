use crate::{abstract_structs, bitwise};

pub struct SparseTable<'a, S> {
    sg: abstract_structs::Semigroup<'a, S>,
    data: Vec<Vec<S>>,
}

impl<'a, S: Default + Clone> SparseTable<'a, S> {
    pub fn new(sg: abstract_structs::Semigroup<'a, S>, a: &Vec<S>) -> Self {
        assert!(sg.idempotent && sg.commutative);
        let n = a.len();
        assert!(n > 0);
        let k = std::cmp::max(1, bitwise::bit_length(n - 1) as usize);
        let mut data = vec![vec![S::default(); n]; k];
        data[0] = a.clone();
        for i in 0..k - 1 {
            data[i + 1] = data[i].clone();
            for j in 0..n - (1 << i) {
                data[i + 1][j] = (sg.operate)(&data[i][j], &data[i][j + (1 << i)])
            }
        }
        Self {
            sg: sg,
            data: data,
        }
    }

    pub fn get(&self, l: usize, r: usize) -> S {
        assert!(l < r && r <= self.data[0].len());
        if r - l == 1 {
            return self.data[0][l].clone();
        }
        let k = bitwise::bit_length(r - 1 - l) as usize - 1;
        (self.sg.operate)(&self.data[k][l], &self.data[k][r - (1 << k)])
    }
}

pub struct DisjointSparseTable<'a, S> {
    sg: abstract_structs::Semigroup<'a, S>,
    data: Vec<Vec<S>>,
}

impl<'a, S: Default + Clone> DisjointSparseTable<'a, S> {
    pub fn new(sg: abstract_structs::Semigroup<'a, S>, a: &Vec<S>) -> Self {
        let n = a.len();
        assert!(n > 0);
        let k = std::cmp::max(1, bitwise::bit_length(n - 1) as usize);
        let mut data = vec![vec![S::default(); n]; k];
        data[0] = a.clone();
        for i in 1..k {
            data[i] = a.clone();
            for j in (1 << i..n + 1).step_by(2 << i) {
                for k in 1..(1 << i) {
                    data[i][j - k - 1] = (sg.operate)(&data[i][j - k - 1], &data[i][j - k]);
                }
                for k in 0..(1 << i) - 1 {
                    if j + k + 1 >= n {
                        break;
                    }
                    data[i][j + k + 1] = (sg.operate)(&data[i][j + k], &data[i][j + k + 1]);
                }
            }
        }
        Self {
            sg: sg,
            data: data,
        }
    }

    pub fn get(&self, l: usize, r: usize) -> S {
        assert!(l < r && r <= self.data[0].len());
        if r - l == 1 {
            return self.data[0][l].clone();
        }
        let log = bitwise::bit_length(l ^ (r - 1)) as usize - 1;
        (self.sg.operate)(&self.data[log][l], &self.data[log][r - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_table() {
        let sg = abstract_structs::Semigroup::<i64> {
            operate: &|x, y| std::cmp::min(*x, *y),
            commutative: true,
            idempotent: true,
        };
        let a = vec![0, 4, 2, 8, 5, 1];
        let sp = SparseTable::new(sg, &a);
        assert_eq!(sp.get(0, 4), 0);
        assert_eq!(sp.get(3, 4), 8);
        assert_eq!(sp.get(1, 6), 1);
    }

    #[test]
    fn test_disjoint_sparse_table() {
        let sg = abstract_structs::Semigroup::<i64> {
            operate: &|x, y| std::cmp::min(*x, *y),
            commutative: true,
            idempotent: true,
        };
        let a = vec![0, 4, 2, 8, 5, 1];
        let sp = DisjointSparseTable::new(sg, &a);
        assert_eq!(sp.get(0, 4), 0);
        assert_eq!(sp.get(3, 4), 8);
        assert_eq!(sp.get(1, 6), 1);
    }
}
