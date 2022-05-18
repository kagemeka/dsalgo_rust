pub fn reduce<T, F>(f: F, v: Vec<T>) -> T
where
    F: Fn(T, T) -> T,
{
    assert!(v.len() > 0);
    v.into_iter().reduce(f).unwrap()
}
