use crate::priority_queue::{Pop, Push};

pub trait DijkstraSparseQueue:
    Push<T = std::cmp::Reverse<(u64, usize)>>
    + Pop<T = std::cmp::Reverse<(u64, usize)>>
    + Default
{
}

impl<T> DijkstraSparseQueue for T where
    T: Push<T = std::cmp::Reverse<(u64, usize)>>
        + Pop<T = std::cmp::Reverse<(u64, usize)>>
        + Default
{
}
