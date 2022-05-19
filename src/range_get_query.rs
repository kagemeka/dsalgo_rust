pub trait RangeGetQuery<S, Id> {
    fn get_range(&mut self, l: usize, r: usize) -> S;
}
