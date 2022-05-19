use crate::{
    commutative_property::CommutativeProperty,
    idempotence::Idempotence,
    range_get_query::RangeGetQuery,
    semigroup::Semigroup,
    sparse_table::SparseTable,
};

impl<G, Id> RangeGetQuery<G::S, Id> for SparseTable<G, Id>
where
    G: Semigroup<Id> + Idempotence<Id> + CommutativeProperty<Id>,
    G::S: Clone,
{
    fn get_range(&mut self, l: usize, r: usize) -> G::S { self.reduce(l, r) }
}
