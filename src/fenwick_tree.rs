use crate::abstract_traits::{AbelianGroup, Additive, Commutative, Monoid};

/// Node Indices
/// (case $|given array| = 8$,
/// internally 1-indexed implemetation)
/// |8              |
/// |4      |       |
/// |2  |   |6  |   |
/// |1| |3| |5| |7| |
pub struct FenwickTree<M: Monoid<S, T> + Commutative<S, T>, S = M, T = Additive> {
    phantom_t: std::marker::PhantomData<T>,
    phantom_m: std::marker::PhantomData<M>,
    data: Vec<S>,
}

impl<M: Monoid<S, T> + Commutative<S, T>, S: Clone, T> From<&[S]> for FenwickTree<M, S, T> {
    fn from(slice: &[S]) -> Self {
        let size = slice.len();
        let mut data = vec![M::identity(); size + 1];
        data[1..].clone_from_slice(slice);
        for node_index in 1..size as isize {
            let parent_node_index = (node_index + (node_index & -node_index)) as usize;
            if parent_node_index <= size {
                data[parent_node_index] =
                    M::operate(&data[parent_node_index], &data[node_index as usize]);
            }
        }
        Self {
            phantom_t: std::marker::PhantomData,
            phantom_m: std::marker::PhantomData,
            data,
        }
    }
}

impl<M: Monoid<S, T> + Commutative<S, T>, S, T> FenwickTree<M, S, T> {
    pub fn new(size: usize) -> Self
    where
        S: Clone,
    {
        (&vec![M::identity(); size]).as_slice().into()
    }

    pub fn size(&self) -> usize { self.data.len() - 1 }

    pub fn set(&mut self, array_index: usize, x: &S) {
        assert!(array_index < self.size());
        let mut node_index = array_index + 1;
        while node_index <= self.size() {
            self.data[node_index] = M::operate(&self.data[node_index], x);
            node_index += (node_index as isize & -(node_index as isize)) as usize;
        }
    }

    fn get(&self, right: usize) -> S {
        assert!(right <= self.size());
        let mut value = M::identity();
        let mut node_index = right;
        while node_index > 0 {
            value = M::operate(&value, &self.data[node_index]);
            node_index -= (node_index as isize & -(node_index as isize)) as usize;
        }
        value
    }

    // pub fn max_right<F>(&self, is_ok: &F) -> usize
    // where
    //     F: Fn(&S) -> bool,
    // {
    // }
}

impl<G: AbelianGroup<S, T>, S, T> FenwickTree<G, S, T> {
    pub fn get_range(&self, left: usize, right: usize) -> S {
        assert!(left <= right);
        G::operate(&G::invert(&self.get(left)), &self.get(right))
    }
}

//     pub fn max_right(&self, is_ok: Box<dyn Fn(&S) -> bool>)
// -> usize {         let n = self.data.len();
//         let mut l = 1;
//         while l << 1 < n {
//             l <<= 1;
//         }
//         let mut v = (self.m.e)();
//         let mut i = 0usize;
//         while l > 0 {
//             if i + l < n && is_ok(&(self.m.op)(&v,
// &self.data[i + l])) {                 i += l;
//                 v = (self.m.op)(&v, &self.data[i + 1]);
//             }
//             l >>= 1;
//         }
//         i
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_fenwick_tree() {
//         let op = |x: &i32, y: &i32| x + y;
//         let e = || 0;
//         let m =
// abstraction::structure::structs::Monoid::<i32> {
// op: &op,             e: &e,
//             commutative: true,
//             idempotent: false,
//         };
//         let mut a = vec![0i32; 10];
//         for i in 0..10i32 {
//             a[i as usize] = i;
//         }
//         let mut fw = FenwickTree::from_vec(m, &a);
//         assert_eq!(fw.get(10), 45);
//         assert_eq!(
//             fw.data,
//             vec![
//                 0, 0, 1, 2, 6, 4, 9, 6, 28, 8, 17
//             ],
//         );
//         println!("{:?}", fw);
//         fw.set(4, &4);
//         assert_eq!(fw.get(8) - fw.get(3), 29);
//     }
// }
