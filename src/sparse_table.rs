use crate::algebra::{bit::bit_length, abstract_::structure::structs::Semigroup};




/// Sparse Table 
/// references
/// - https://cp-algorithms.com/data_structures/sparse-table.html
pub struct SparseTable<'a, S> {
    sg: Semigroup<'a, S>,
    data: Vec<Vec<S>>, 
}


impl<'a, S: Default + Clone> SparseTable<'a, S> {
    /// O(N\log{N})
    pub fn new(sg: Semigroup<'a, S>, a: &Vec<S>) -> Self {
        assert!(sg.idempotent && sg.commutative);
        let n = a.len();
        assert!(n > 0);
        let k = std::cmp::max(1, bit_length(n - 1));
        let mut data = vec![vec![S::default(); n]; k];
        data[0] = a.clone();
        for i in 0..k - 1 {
            data[i + 1] = data[i].clone();
            for j in 0..n - (1 << i) {
                data[i + 1][j] = (sg.op)(&data[i][j], &data[i][j + (1 << i)])
            }
        }   
        Self { sg: sg, data: data }     
    }

    /// O(1)
    pub fn get(&self, l: usize, r: usize) -> S {
        assert!(l < r && r <= self.data[0].len());
        let k = bit_length(r - l) - 1;
        (self.sg.op)(&self.data[k][l], &self.data[k][r - (1 << k)])
    }
}




/// Disjoint Sparse Table 
/// references
/// - https://noshi91.hatenablog.com/entry/2018/05/08/183946
/// - https://github.com/noshi91/NoshiMochiLibrary/blob/master/SparseTable/DisjointSparseTable.noshi.cpp
pub struct DisjointSparseTable<'a, S> {
    sg: Semigroup<'a, S>,
    data: Vec<Vec<S>>, 
}


impl<'a, S: Default + Clone> DisjointSparseTable<'a, S> {
    pub fn new(sg: Semigroup<'a, S>, a: &Vec<S>) -> Self {
        let n = a.len();
        assert!(n > 0);
        let k = std::cmp::max(1, bit_length(n - 1));
        let mut data = vec![vec![S::default(); n]; k];
        data[0] = a.clone();
        for i in 1..k {
            data[i] = a.clone();
            for j in (1 << i..n + 1).step_by(2 << i) {
                for k in 1..(1 << i) {
                    data[i][j - k - 1] = (sg.op)(&data[i][j - k - 1], &data[i][j - k]);
                }
                for k in 0..(1 << i) - 1 {
                    if j + k + 1 >= n { break; }
                    data[i][j + k + 1] = (sg.op)(&data[i][j + k], &data[i][j + k + 1]);
                }
            }
        }
        Self { sg: sg, data: data }
    }

    pub fn get(&self, l: usize, r: usize) -> S {
        assert!(l < r && r <= self.data[0].len());
        if r - l == 1 { return self.data[0][l].clone(); }
        let k = bit_length(l ^ r) - 1;
        (self.sg.op)(&self.data[k][l], &self.data[k][r - 1])
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_table() {
        let sg = Semigroup::<i64> {
            op: &|x, y| std::cmp::min(*x, *y),
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
        let sg = Semigroup::<i64> {
            op: &|x, y| std::cmp::min(*x, *y),
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