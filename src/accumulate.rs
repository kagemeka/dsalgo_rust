pub fn accumulate<T, F>(mut v: Vec<T>, f: F) -> Vec<T>
where
    T: Clone,
    F: Fn(T, T) -> T,
{
    for i in 0..v.len() - 1 {
        v[i + 1] = f(v[i].clone(), v[i + 1].clone());
    }
    v
}
