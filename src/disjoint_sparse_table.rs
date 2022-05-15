use crate::{
    bit_length::bit_length,
    group_theory::{CommutativeProperty, Idempotence, Semigroup},
};



pub struct DisjointSparseTable<S, I>
where
    S: Semigroup<I> + CommutativeProperty<S, I>,
    I: crate::group_theory::BinaryOperationIdentifier,
{
    phantom: std::marker::PhantomData<I>,
    data: Vec<Vec<S>>,
}

impl<S, I> DisjointSparseTable<S, I>
where
    S: Semigroup<I> + CommutativeProperty<S, I> + Copy,
    I: crate::group_theory::BinaryOperationIdentifier,
{
    pub fn new(slice: &[S]) -> Self {
        let width = slice.len();
        let height = if width <= 1 {
            1
        } else {
            bit_length((width - 1) as u64) as usize
        };
        let mut data = vec![slice.to_vec()];
        for log in 1..height {
            data.push(slice.to_vec());
            // store cummulative products from borders.
            for border in (1 << log..width + 1).step_by(2 << log) {
                for delta in 1..(1 << log) {
                    // prod to left.
                    let index = border - delta;
                    // data[log][index - 1] = S::operate(&data[log][index - 1],
                    // &data[log][index]);
                    data[log][index - 1] =
                        data[log][index - 1].operate(data[log][index]);
                }
                for delta in 0..(1 << log) - 1 {
                    // prod to right
                    let index = border + delta;
                    if index + 1 >= width {
                        // for last sequence
                        break;
                    }
                    // data[log][index + 1] = S::operate(&data[log][index],
                    // &data[log][index + 1]);
                    data[log][index + 1] =
                        data[log][index].operate(data[log][index + 1]);
                }
            }
        }
        Self {
            phantom: std::marker::PhantomData,
            data,
        }
    }

    pub fn get_range(&self, left: usize, right: usize) -> S {
        assert!(left < right && right <= self.data[0].len());
        if right - left == 1 {
            return self.data[0][left];
        }
        let log = bit_length((left ^ (right - 1)) as u64) as usize - 1;
        // S::operate(&self.data[log][left], &self.data[log][right -
        // 1])
        self.data[log][left].operate(self.data[log][right - 1])
    }
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_self_as_min() {
//         use crate::{
//             associative_property::AssociativeProperty,
//             binary_operation::BinaryOperation,
//             commutative_property::CommutativeProperty,
//             idempotence::Idempotence,
//         };

//         struct Min;

//         impl BinaryOperation<usize, usize, usize, Min> for usize {
//             fn operate(lhs: usize, rhs: usize) -> usize {
//                 std::cmp::min(lhs, rhs)
//             }
//         }
//         impl AssociativeProperty<usize, Min> for usize {}
//         impl Idempotence<usize, Min> for usize {}
//         impl CommutativeProperty<usize, usize, Min> for usize {}

//         let arr: Vec<usize> = vec![0, 4, 2, 8, 5, 1];
//         let sp = super::SparseTable::<usize, Min, usize>::new(&arr);
//         assert_eq!(sp.fold(0, 4), 0);
//         assert_eq!(sp.fold(3, 4), 8);
//         assert_eq!(sp.fold(1, 6), 1);
//     }
// }
