use crate::{
    disjoint_sparse_table::DisjointSparseTable,
    group_theory_id::Min,
    range_minimum_query::RangeMinimumQuery,
};

impl RangeMinimumQuery<(usize, usize)>
    for DisjointSparseTable<(usize, usize), Min, (usize, usize)>
{
    fn query(&mut self, l: usize, r: usize) -> (usize, usize) {
        self.fold(l, r)
    }
}
