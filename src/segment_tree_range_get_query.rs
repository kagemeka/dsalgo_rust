use crate::{
    monoid::Monoid,
    range_get_query::RangeGetQuery,
    segment_tree::SegmentTree,
};

impl<M, Id> RangeGetQuery<M::S, Id> for SegmentTree<M, Id>
where
    M: Monoid<Id>,
    M::S: Clone,
{
    fn get_range(&mut self, l: usize, r: usize) -> M::S { self.reduce(l, r) }
}
