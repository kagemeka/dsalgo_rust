use crate::{
    group_theory_id::Min,
    range_minimum_query::RangeMinimumQuery,
    sqrt_decomposition::SqrtDecomposition,
};

impl RangeMinimumQuery<(usize, usize)>
    for SqrtDecomposition<(usize, usize), Min, (usize, usize)>
{
    fn query(&mut self, l: usize, r: usize) -> (usize, usize) {
        self.fold(l, r)
    }
}
