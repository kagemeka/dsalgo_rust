use crate::accumulate::accumulate;

pub fn factorial_table<T>(size: usize) -> Vec<T>
where
    T: std::ops::Mul<Output = T> + From<u64> + Clone,
{
    if size == 0 {
        return vec![];
    }
    let mut v = (0..size as u64).map(|i| i.into()).collect::<Vec<T>>();
    v[0] = 1.into();
    let op = |a: T, b: T| -> T { a * b };
    accumulate(v, op)
}
