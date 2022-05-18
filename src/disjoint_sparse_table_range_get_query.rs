use crate::{
    commutative_property::CommutativeProperty,
    disjoint_sparse_table::DisjointSparseTable,
    range_get_query::RangeGetQuery,
    semigroup::Semigroup,
};

impl<S, G, I> RangeGetQuery<S, I> for DisjointSparseTable<S, G, I>
where
    G: Semigroup<S, I> + CommutativeProperty<S, S, I>,
    S: Clone,
{
    fn get_range(&mut self, l: usize, r: usize) -> S { self.reduce(l, r) }
}
