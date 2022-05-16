pub trait Choose<T> {
    fn choose(&mut self, n: usize, k: usize) -> T;
}
