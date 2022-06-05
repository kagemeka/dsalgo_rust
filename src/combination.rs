use crate::{
    factorial_table::factorial_table,
    inverse_factorial_table::inverse_factorial_table,
    multiplicative_inverse::MulInv,
};

pub struct Combination<T> {
    fact: Vec<T>,
    inv_fact: Vec<T>,
}

impl<T> Combination<T>
where
    T: std::ops::Mul<Output = T> + From<u64> + Clone,
{
    pub fn new(size: usize) -> Self
    where
        T: MulInv<Output = T>,
    {
        let fact = factorial_table::<T>(size);
        let inv_fact = inverse_factorial_table::<T>(size);
        Self { fact, inv_fact }
    }

    pub fn calc(&self, n: u64, k: u64) -> T {
        if k > n {
            return 0.into();
        }
        let n = n as usize;
        let k = k as usize;
        self.fact[n].clone()
            * self.inv_fact[n - k].clone()
            * self.inv_fact[k].clone()
    }

    pub fn inv(&self, n: u64, k: u64) -> T {
        assert!(k <= n); // (n, k) := 0 if k > n, so the inverse is undefined.
        let n = n as usize;
        let k = k as usize;
        self.inv_fact[n].clone()
            * self.fact[k].clone()
            * self.fact[n - k].clone()
    }
}

// #[cfg(test)]
mod tests {

    #[test]
    fn test() {
        use super::*;
        use crate::{modular_int::ModularInt, static_modulus::StaticMod};
        type Mint = ModularInt<StaticMod<1_000_000_007>>;
        let choose = Combination::<Mint>::new(100);
        assert_eq!(choose.calc(5, 2), 10.into());
    }
}
