pub fn unique<T: Ord + Clone>(slice: &[T]) -> Vec<T> {
    let mut values = slice.to_vec();
    values.sort();
    values.dedup();
    values
}
