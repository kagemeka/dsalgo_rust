use crate::{
    choose::Choose,
    combination::Combination,
    multiplicative_inverse::MulInv,
};

impl<T> Choose<T> for Combination<T>
where
    T: std::ops::Mul<Output = T> + MulInv<Output = T> + From<usize> + Clone,
{
    fn choose(&self, n: usize, k: usize) -> T { self.calc(n, k).unwrap() }
}
