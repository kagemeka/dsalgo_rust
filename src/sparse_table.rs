use crate::{
    commutative_property::CommutativeProperty,
    idempotence::Idempotence,
    semigroup::Semigroup,
};

// TODO: use Semigroup2 after language update on AtCoder
pub struct SparseTable<S, G, Id> {
    phantom: std::marker::PhantomData<(G, Id)>,
    data: Vec<Vec<S>>,
}

impl<S, G, Id> std::iter::FromIterator<S> for SparseTable<S, G, Id>
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
        Self {
            phantom: std::marker::PhantomData,
            data,
        }
    }
}

impl<S, G, Id> SparseTable<S, G, Id>
where
    G: Semigroup<S, Id> + Idempotence<S, Id> + CommutativeProperty<S, S, Id>,
    S: Clone,
{
    pub fn new(slice: &[S]) -> Self { Self::from_iter(slice.iter().cloned()) }

    pub fn size(&self) -> usize { self.data[0].len() }

    pub fn reduce(&self, l: usize, r: usize) -> S {
        assert!(l < r && r <= self.size());
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
        let sp = super::SparseTable::<usize, usize, Min>::new(&arr);
        assert_eq!(sp.reduce(0, 4), 0);
        assert_eq!(sp.reduce(3, 4), 8);
        assert_eq!(sp.reduce(1, 6), 1);
    }
}
