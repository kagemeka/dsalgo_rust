use crate::{
    commutative_property::CommutativeProperty,
    disjoint_sparse_table::DisjointSparseTable,
    range_get_query::RangeGetQuery,
    semigroup::Semigroup,
};

impl<G, I> RangeGetQuery<G::S, I> for DisjointSparseTable<G, I>
where
    G: Semigroup<I> + CommutativeProperty<I>,
    G::S: Clone,
{
    fn get_range(&mut self, l: usize, r: usize) -> G::S { self.reduce(l, r) }
}
