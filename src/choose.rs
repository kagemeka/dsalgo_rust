pub trait Choose<T> {
    fn choose(&mut self, n: u64, k: u64) -> T;
}
