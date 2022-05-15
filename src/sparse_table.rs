use crate::{
    commutative_property::CommutativeProperty,
    idempotence::Idempotence,
    semigroup::Semigroup,
};

pub struct SparseTable<S, Id, G>
where
    G: Semigroup<S, Id> + Idempotence<S, Id> + CommutativeProperty<S, S, Id>,
{
    phantom_id: std::marker::PhantomData<Id>,
    phandom_g: std::marker::PhantomData<G>,
    data: Vec<Vec<S>>,
}

impl<S, Id, G> std::iter::FromIterator<S> for SparseTable<S, Id, G>
where
    G: Semigroup<S, Id> + Idempotence<S, Id> + CommutativeProperty<S, S, Id>,
    S: Clone,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let mut data = vec![iter.into_iter().collect::<Vec<_>>()];
        let max_width = data[0].len();
        let height = if max_width <= 1 {
            1
        } else {
            max_width.next_power_of_two().trailing_zeros() as usize
        };
        for i in 1..height {
            let row_size = max_width - (1 << i) + 1;
            // last is max_width - (1 << i) covering (1 << i)
            // including the position.
            data.push(
                (0..row_size)
                    .map(|j| {
                        G::operate(
                            data[i - 1][j].clone(),
                            data[i - 1][j + (1 << (i - 1))].clone(),
                        )
                    })
                    .collect(),
            );
        }
        SparseTable {
            phantom_id: std::marker::PhantomData,
            phandom_g: std::marker::PhantomData,
            data,
        }
    }
}

impl<S, Id, G> SparseTable<S, Id, G>
where
    G: Semigroup<S, Id> + Idempotence<S, Id> + CommutativeProperty<S, S, Id>,
    S: Clone,
{
    pub fn new(slice: &[S]) -> Self { Self::from_iter(slice.iter().cloned()) }

    pub fn fold(&self, l: usize, r: usize) -> S {
        assert!(l < r && r <= self.data[0].len());
        if r - l == 1 {
            return self.data[0][l].clone();
        }
        let i = (r - l).next_power_of_two().trailing_zeros() as usize - 1;
        G::operate(
            self.data[i][l].clone(),
            self.data[i][r - (1 << i)].clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_self_as_min() {
        use crate::{
            associative_property::AssociativeProperty,
            binary_operation::BinaryOperation,
            commutative_property::CommutativeProperty,
            idempotence::Idempotence,
        };

        struct Min;

        impl BinaryOperation<usize, usize, usize, Min> for usize {
            fn operate(lhs: usize, rhs: usize) -> usize {
                std::cmp::min(lhs, rhs)
            }
        }
        impl AssociativeProperty<usize, Min> for usize {}
        impl Idempotence<usize, Min> for usize {}
        impl CommutativeProperty<usize, usize, Min> for usize {}

        let arr: Vec<usize> = vec![0, 4, 2, 8, 5, 1];
        let sp = super::SparseTable::<usize, Min, usize>::new(&arr);
        assert_eq!(sp.fold(0, 4), 0);
        assert_eq!(sp.fold(3, 4), 8);
        assert_eq!(sp.fold(1, 6), 1);
    }
}
