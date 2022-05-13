pub fn accumulate<T, F>(mut v: Vec<T>, f: F) -> Vec<T>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    for i in 0..v.len() - 1 {
        v[i + 1] = f(v[i], v[i + 1]);
    }
    v
}
