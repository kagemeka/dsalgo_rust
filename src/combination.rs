use crate::{
    factorial_table::factorial_table,
    inverse_factorial_table::inverse_factorial_table,
    multiplicative_inverse::MulInv,
};

pub struct Combination<T>
where
    T: std::ops::Mul + MulInv<Output = T> + From<usize> + Clone,
{
    fact: Vec<T>,
    inv_fact: Vec<T>,
}

impl<T> Combination<T>
where
    T: std::ops::Mul<Output = T> + MulInv<Output = T> + From<usize> + Clone,
{
    pub fn new(size: usize) -> Self {
        let fact = factorial_table::<T>(size);
        let inv_fact = inverse_factorial_table::<T>(size);
        Self { fact, inv_fact }
    }

    pub fn calc(&self, n: usize, k: usize) -> Result<T, ()> {
        if n < k {
            Ok(0.into())
        } else if n >= self.fact.len() {
            Err(())
        } else {
            Ok(self.fact[n].clone() * self.inv_fact[n - k].clone() * self.inv_fact[k].clone())
        }
    }

    pub fn inv(&self, n: usize, k: usize) -> Result<T, ()> {
        if n < k {
            Ok(0.into())
        } else if n >= self.fact.len() {
            Err(())
        } else {
            Ok(self.inv_fact[n].clone() * self.fact[k].clone() * self.fact[n - k].clone())
        }
    }
}
