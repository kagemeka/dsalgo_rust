use crate::{
    group_theory_id::Min,
    range_minimum_query::RangeMinimumQuery,
    sparse_table::SparseTable,
};

impl RangeMinimumQuery<(usize, usize)>
    for SparseTable<(usize, usize), Min, (usize, usize)>
{
    fn query(&mut self, l: usize, r: usize) -> (usize, usize) {
        self.fold(l, r)
    }
}
