use crate::{
    accumulate::accumulate,
    factorial::factorial,
    multiplicative_inverse::MulInv,
};

pub fn inverse_factorial_table<T>(size: usize) -> Vec<T>
where
    T: std::ops::Mul<Output = T> + MulInv<Output = T> + From<u64> + Clone,
{
    if size == 0 {
        return vec![];
    }
    let mut v = (0..size as u64).map(|i| (i + 1).into()).collect::<Vec<T>>();
    if size == 0 {
        return v;
    }
    v[size - 1] = factorial::<T>(size as u64 - 1).mul_inv();
    let op = |a: T, b: T| -> T { a * b };
    let mut ifact = accumulate(&op, v.into_iter().rev()).collect::<Vec<_>>();
    ifact.reverse();
    ifact
}
