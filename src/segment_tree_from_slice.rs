use crate::{monoid::Monoid, segment_tree::SegmentTree};

impl<S, M, Id> From<&[S]> for SegmentTree<S, M, Id>
where
    M: Monoid<S, Id>,
    S: Clone,
{
    fn from(slice: &[S]) -> Self { Self::from_iter(slice.iter().cloned()) }
}

#[cfg(test)]
mod tests {}
