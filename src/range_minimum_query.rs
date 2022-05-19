use crate::{group_theory_id::Min, range_get_query::RangeGetQuery};

pub trait RangeMinimumQuery<S> {
    fn range_minimum(&mut self, l: usize, r: usize) -> S;
}

impl<S, T> RangeMinimumQuery<S> for T
where
    T: RangeGetQuery<S, Min>,
{
    fn range_minimum(&mut self, l: usize, r: usize) -> S {
        self.get_range(l, r)
    }
}
