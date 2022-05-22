use crate::priority_queue::{MinimumQueue, Pop, Push};

pub trait DijkstraSparseQueue:
    MinimumQueue + Push<T = (u64, usize)> + Pop<T = (u64, usize)> + Default
{
}

impl<Q> DijkstraSparseQueue for Q where
    Q: MinimumQueue + Push<T = (u64, usize)> + Pop<T = (u64, usize)> + Default
{
}
