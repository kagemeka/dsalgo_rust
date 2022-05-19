use crate::{
    range_get_query::RangeGetQuery,
    semigroup::Semigroup,
    sqrt_decomposition::SqrtDecomposition,
};

impl<G, Id> RangeGetQuery<G::S, Id> for SqrtDecomposition<G, Id>
where
    G: Semigroup<Id>,
    G::S: Clone,
{
    fn get_range(&mut self, l: usize, r: usize) -> G::S {
        self.fast_reduce(l, r)
    }
}
