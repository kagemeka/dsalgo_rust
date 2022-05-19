use crate::{
    bit_length::bit_length,
    commutative_property::CommutativeProperty,
    semigroup::Semigroup,
};

pub struct DisjointSparseTable<G: Semigroup<Id>, Id> {
    data: Vec<Vec<G::S>>,
}

impl<G, Id> std::iter::FromIterator<G::S> for DisjointSparseTable<G, Id>
where
    G: Semigroup<Id> + CommutativeProperty<Id>,
    G::S: Clone,
{
    fn from_iter<T: IntoIterator<Item = G::S>>(iter: T) -> Self {
        let mut data = vec![iter.into_iter().collect::<Vec<_>>()];
        let size = data[0].len();
        let height = if size <= 1 {
            1
        } else {
            size.next_power_of_two().trailing_zeros() as usize
        };
        for i in 1..height {
            let mut row = data[0].clone();
            for p in (1 << i..=size).step_by(2 << i) {
                for d in 1..(1 << i) {
                    let j = p - d;
                    row[j - 1] = G::operate(
                        row[j - 1].clone(),
                        row[j].clone(),
                    );
                }
                for d in 0..(1 << i) - 1 {
                    let j = p + d;
                    if j + 1 >= size {
                        break;
                    }
                    row[j + 1] = G::operate(
                        row[j].clone(),
                        row[j + 1].clone(),
                    );
                }
            }
            data.push(row);
        }
        Self { data }
    }
}

impl<G, Id> DisjointSparseTable<G, Id>
where
    G: Semigroup<Id> + CommutativeProperty<Id>,
    G::S: Clone,
{
    pub fn new(slice: &[G::S]) -> Self {
        Self::from_iter(slice.iter().cloned())
    }

    pub fn size(&self) -> usize { self.data[0].len() }

    /// [l, r)
    pub fn reduce(&self, l: usize, mut r: usize) -> G::S {
        assert!(l < r && r <= self.size());
        r -= 1; // internally, consider [l, r]
        if l == r {
            return self.data[0][l].clone();
        }
        let i = bit_length((l ^ r) as u64) as usize - 1;
        // if i = 0, then use 0-th row.
        // if i = 3, then use 3-th row.
        // what does this mean?
        // only msb of l \xor r is important.
        // because,
        // for each bit j (checking in descending order from top bit),
        // if for any k in 2^j..=|data| (step 2^{j + 1}), l < k <= r,
        // then j-th bit of l \xor r is gonna be 1.
        // so the query can be dealed with j-th row.
        // <->
        // if j-th bit of l \xor r is 0,
        // then for all k in 2^j..=|data| (step 2^{j + 1}),
        // k <= l < r or l < r < k.
        // so the query cannot be dealed with j-th row.
        // then, check {j-1}-th bit next...
        G::operate(
            self.data[i][l].clone(),
            self.data[i][r].clone(),
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
        };

        struct Min;

        impl BinaryOperation<Min> for usize {
            type Codomain = Self;
            type Lhs = Self;
            type Rhs = Self;

            fn map(lhs: usize, rhs: usize) -> usize { std::cmp::min(lhs, rhs) }
        }
        impl AssociativeProperty<Min> for usize {}
        impl CommutativeProperty<Min> for usize {}

        let arr: Vec<usize> = vec![0, 4, 2, 8, 5, 1];
        let sp = super::DisjointSparseTable::<usize, Min>::new(&arr);
        assert_eq!(sp.reduce(0, 4), 0);
        assert_eq!(sp.reduce(3, 4), 8);
        assert_eq!(sp.reduce(1, 6), 1);
    }
}
