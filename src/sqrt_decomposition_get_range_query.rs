use crate::{
    range_get_query::RangeGetQuery,
    semigroup::Semigroup,
    sqrt_decomposition::SqrtDecomposition,
};

impl<S, G, Id> RangeGetQuery<S, Id> for SqrtDecomposition<S, G, Id>
where
    G: Semigroup<S, Id>,
    S: Clone,
{
    fn get_range(&mut self, l: usize, r: usize) -> S { self.reduce(l, r) }
}
