use crate::{
    commutative_property::CommutativeProperty,
    idempotence::Idempotence,
    range_get_query::RangeGetQuery,
    semigroup::Semigroup,
    sparse_table::SparseTable,
};

impl<S, G, Id> RangeGetQuery<S, Id> for SparseTable<S, G, Id>
where
    G: Semigroup<S, Id> + Idempotence<S, Id> + CommutativeProperty<S, S, Id>,
    S: Clone,
{
    fn get_range(&mut self, l: usize, r: usize) -> S { self.reduce(l, r) }
}
