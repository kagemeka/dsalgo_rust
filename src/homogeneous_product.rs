use crate::{combination::Combination, multiplicative_inverse::MulInv};

pub struct HomogeneousProduct<T>
where
    T: std::ops::Mul<Output = T> + MulInv<Output = T> + From<usize> + Clone,
{
    choose: Combination<T>,
}

impl<T> HomogeneousProduct<T>
where
    T: std::ops::Mul<Output = T> + MulInv<Output = T> + From<usize> + Clone,
{
    pub fn new(size: usize) -> Self { Self { choose: Combination::new(size) } }

    pub fn calc(&self, n: usize, k: usize) -> T { if n == 0 { 0.into() } else { self.choose.calc(n + k - 1, k) } }
}
