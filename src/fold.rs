pub fn fold_left<S, T, F>(f: F, init: S, v: Vec<T>) -> S
where
    F: Fn(S, T) -> S,
{
    v.into_iter().fold(init, f)
}
