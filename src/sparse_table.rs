use crate::{
    bitwise,
    group_theory::{CommutativeProperty, Idempotence, Semigroup},
};
pub struct SparseTable<S, I>
where
    S: Semigroup<I> + Idempotence<I> + CommutativeProperty<S, I>,
    I: crate::group_theory::BinaryOperationIdentifier,
{
    phantom: std::marker::PhantomData<I>,
    data: Vec<Vec<S>>,
}

impl<S, I> SparseTable<S, I>
where
    S: Semigroup<I> + CommutativeProperty<S, I> + Idempotence<I> + Copy,
    I: crate::group_theory::BinaryOperationIdentifier,
{
    pub fn new(slice: &[S]) -> Self {
        let max_width = slice.len();
        let height = if max_width <= 1 {
            1
        } else {
            bitwise::bit_length(max_width - 1) as usize
        };
        let mut data = vec![slice.to_vec()];
        for log in 1..height {
            let row_size = max_width - (1 << log) + 1;
            data.push(
                (0..row_size)
                    .map(|index| {
                        // S::operate(&data[log - 1][index], &data[log - 1][index + (1
                        // << (log - 1))])
                        data[log - 1][index].operate(data[log - 1][index + (1 << (log - 1))])
                    })
                    .collect(),
            );
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
        let log = bitwise::bit_length(right - 1 - left) as usize - 1;
        // S::operate(&self.data[log][left], &self.data[log][right - (1
        // << log)])
        self.data[log][left].operate(self.data[log][right - (1 << log)])
    }
}

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
            bitwise::bit_length(width - 1) as usize
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
                    data[log][index - 1] = data[log][index - 1].operate(data[log][index]);
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
                    data[log][index + 1] = data[log][index].operate(data[log][index + 1]);
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
        let log = bitwise::bit_length(left ^ (right - 1)) as usize - 1;
        // S::operate(&self.data[log][left], &self.data[log][right -
        // 1])
        self.data[log][left].operate(self.data[log][right - 1])
    }
}

#[cfg(test)]
mod tests {
    use crate::group_theory;

    #[test]
    fn test_self_as_min() {
        let arr: Vec<usize> = vec![0, 4, 2, 8, 5, 1];

        struct Min;
        impl crate::group_theory::BinaryOperationIdentifier for Min {}

        // impl group_theory::BinaryOperation<Min> for usize {
        impl group_theory::BinaryOperation<Self, Self, Min> for usize {
            fn operate(self, other: Self) -> Self { std::cmp::min(self, other) }
        }
        impl group_theory::AssociativeProperty<Min> for usize {}
        impl group_theory::Idempotence<Min> for usize {}
        impl group_theory::CommutativeProperty<Self, Min> for usize {}
        let sp = super::SparseTable::<usize, Min>::new(&arr);
        assert_eq!(sp.get_range(0, 4), 0);
        assert_eq!(sp.get_range(3, 4), 8);
        assert_eq!(sp.get_range(1, 6), 1);
        let sp = super::DisjointSparseTable::<usize, Min>::new(&arr);
        assert_eq!(sp.get_range(0, 4), 0);
        assert_eq!(sp.get_range(3, 4), 8);
        assert_eq!(sp.get_range(1, 6), 1);
    }
}
