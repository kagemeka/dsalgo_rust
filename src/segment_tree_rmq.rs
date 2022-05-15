use crate::{
    group_theory_id::Min,
    range_minimum_query::RangeMinimumQuery,
    segment_tree::SegmentTree,
};

impl RangeMinimumQuery<(usize, usize)>
    for SegmentTree<(usize, usize), Min, (usize, usize)>
{
    fn query(&mut self, l: usize, r: usize) -> (usize, usize) {
        self.fold(l, r)
    }
}
