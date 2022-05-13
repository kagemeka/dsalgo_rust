use crate::{
    factorial_table::factorial_table,
    inverse_factorial_table::inverse_factorial_table,
    multiplicative_inverse::MulInv,
};

pub struct Combination<T>
where
    T: std::ops::Mul + MulInv<Output = T> + From<usize> + Copy,
{
    fact: Vec<T>,
    inv_fact: Vec<T>,
}

impl<T> Combination<T>
where
    T: std::ops::Mul<Output = T> + MulInv<Output = T> + From<usize> + Copy,
{
    pub fn new(size: usize) -> Self {
        let fact = factorial_table::<T>(size);
        let inv_fact = inverse_factorial_table::<T>(size);
        Self { fact, inv_fact }
    }

    pub fn calc(&self, n: usize, k: usize) -> T {
        if n < k {
            0.into()
        } else {
            self.fact[n] * self.inv_fact[k] * self.inv_fact[n - k]
        }
    }

    pub fn inv(&self, n: usize, k: usize) -> T {
        if n < k {
            0.into()
        } else {
            self.inv_fact[n] * self.fact[k] * self.fact[n - k]
        }
    }
}
