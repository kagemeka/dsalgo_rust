use crate::{
    monoid::Monoid,
    range_get_query::RangeGetQuery,
    segment_tree::SegmentTree,
};

impl<S, M, Id> RangeGetQuery<S, Id> for SegmentTree<S, M, Id>
where
    M: Monoid<S, Id>,
    S: Clone,
{
    fn get_range(&mut self, l: usize, r: usize) -> S { self.reduce(l, r) }
}
