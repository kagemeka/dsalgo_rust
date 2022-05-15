use crate::{monoid::Monoid, segment_tree::SegmentTree};

impl<S, Id, M> From<&[S]> for SegmentTree<S, Id, M>
where
    Self: std::iter::FromIterator<S>,
    S: Clone,
    M: Monoid<S, Id>,
{
    fn from(slice: &[S]) -> Self { Self::from_iter(slice.iter().cloned()) }
}
