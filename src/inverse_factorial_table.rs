use crate::{accumulate::accumulate, factorial::factorial, multiplicative_inverse::MulInv};

pub fn inverse_factorial_table<T>(size: usize) -> Vec<T>
where
    T: std::ops::Mul<Output = T> + MulInv<Output = T> + From<usize> + Copy,
{
    let mut v = (0..size).map(|i| (i + 1).into()).collect::<Vec<T>>();
    v[size - 1] = factorial::<T>(size - 1).mul_inv();
    let op = |a: T, b: T| -> T { a * b };
    v.reverse();
    v = accumulate(v, op);
    v.reverse();
    v
}
