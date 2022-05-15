pub trait RangeMinimumQuery<T> {
    fn query(&mut self, left: usize, right: usize) -> T;
}
