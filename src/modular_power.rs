use crate::{
    modular::Modular,
    modulus::Modulus,
    power_monoid::{pow_monoid, PowerMonoid},
};

pub fn modular_pow(modulus: u32, base: u64, exponent: u64) -> u32 {
    let modulus = modulus as u64;
    pow_monoid(
        &|x, y| x * y % modulus,
        &|| 1,
        base % modulus,
        exponent,
    ) as u32
}

impl<M: Modulus + Clone> Modular<M> {
    pub fn pow(self, exponent: u64) -> Self { self.pow_monoid(exponent) }
}