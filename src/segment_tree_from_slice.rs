use crate::{monoid::Monoid, segment_tree::SegmentTree};

impl<M, Id> From<&[M::S]> for SegmentTree<M, Id>
where
    M: Monoid<Id>,
    M::S: Clone,
{
    fn from(slice: &[M::S]) -> Self { Self::from_iter(slice.iter().cloned()) }
}

#[cfg(test)]
mod tests {}
