// use crate::{abstract_traits, bitset};
// pub struct SparseTable<S> {
//     data: Vec<Vec<S>>,
// }

// impl<S: abstract_traits::Semigroup + Idempotent>
// SparseTable<S> where S::Identity: Clone {
//     pub fn new(arr: &Vec<S>) -> Self {
//         assert!(sg.idempotent && sg.commutative);
//         let n = a.len();
//         assert!(n > 0);
//         let k = std::cmp::max(1, bitset::bit_length(n - 1));
//         let mut data = vec![vec![S::default(); n]; k];
//         data[0] = a.clone();
//         for i in 0..k - 1 {
//             data[i + 1] = data[i].clone();
//             for j in 0..n - (1 << i) {
//                 data[i + 1][j] =
//                     (sg.operate)(&data[i][j], &data[i][j +
// (1 << i)])             }
//         }
//         Self {
//             sg: sg,
//             data: data,
//         }
//     }

//     pub fn get(&self, l: usize, r: usize) -> S {
//         assert!(l < r && r <= self.data[0].len());
//         if r - l == 1 {
//             return self.data[0][l].clone();
//         }
//         let k = bitset::bit_length(r - 1 - l) - 1;
//         (self.sg.operate)(&self.data[k][l], &self.data[k][r
// - (1 << k)])     }
// }
