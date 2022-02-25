use crate::{
    abstract_traits_2::{Additive, Commutative, Idempotent, Semigroup},
    bitset,
};
pub struct SparseTable<Sg, S = Sg, T = Additive>
where
    Sg: Semigroup<S, T> + Idempotent<S, T> + Commutative<S, T>,
{
    phantom_sg: std::marker::PhantomData<Sg>,
    phantom_t: std::marker::PhantomData<T>,
    data: Vec<Vec<S>>,
}

impl<Sg, S, T> SparseTable<Sg, S, T>
where
    Sg: Semigroup<S, T> + Idempotent<S, T> + Commutative<S, T>,
    S: Copy,
{
    pub fn new(arr: &Vec<S>) -> Self {
        let n = arr.len();
        assert!(n > 0);
        let mut data = vec![arr.clone()];
        for i in 1..bitset::bit_length(n - 1) {
            let row_size = n - (1 << i) + 1;
            data.push(data[i - 1][..row_size].to_vec());
            for j in 0..row_size {
                data[i][j] = Sg::operate(
                    &data[i - 1][j],
                    &data[i - 1][j + (1 << (i - 1))],
                );
            }
        }
        Self {
            phantom_sg: std::marker::PhantomData,
            phantom_t: std::marker::PhantomData,
            data: data,
        }
    }

    pub fn get(&self, left: usize, right: usize) -> S {
        assert!(left < right && right <= self.data[0].len());
        if right - left == 1 {
            return self.data[0][left];
        }
        let k = bitset::bit_length(right - 1 - left) - 1;
        Sg::operate(&self.data[k][left], &self.data[k][right - (1 << k)])
    }
}

#[cfg(test)]
mod tests {
    use crate::abstract_traits_2;

    #[test]
    fn test_sparse_table() {
        let arr: Vec<usize> = vec![0, 4, 2, 8, 5, 1];

        struct Min;
        impl abstract_traits_2::BinaryOperation<usize, Min> for usize {
            fn operate(lhs: &Self, rhs: &Self) -> Self {
                std::cmp::min(*lhs, *rhs)
            }
        }
        impl abstract_traits_2::Idempotent<usize, Min> for usize {}
        impl abstract_traits_2::Commutative<usize, Min> for usize {}
        let sp = super::SparseTable::<usize, _, Min>::new(&arr);
        assert_eq!(sp.get(0, 4), 0);
        assert_eq!(sp.get(3, 4), 8);
        assert_eq!(sp.get(1, 6), 1);
    }

    // #[test]
    // fn test_disjoint_sparse_table() {
    //     let sg = abstract_structs::Semigroup::<i64> {
    //         operate: &|x, y| std::cmp::min(*x, *y),
    //         commutative: true,
    //         idempotent: true,
    //     };
    //     let a = vec![0, 4, 2, 8, 5, 1];
    //     let sp = DisjointSparseTable::new(sg, &a);
    //     assert_eq!(sp.get(0, 4), 0);
    //     assert_eq!(sp.get(3, 4), 8);
    //     assert_eq!(sp.get(1, 6), 1);
    // }
}
