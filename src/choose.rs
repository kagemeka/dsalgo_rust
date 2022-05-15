pub trait Choose<T> {
    fn choose(&self, n: usize, k: usize) -> T;
}
