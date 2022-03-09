use crate::modular_static::{IsPrime, Modular, Modulus};

pub trait ConstantModulus: Modulus {
    const VALUE: usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MOD998_244_353;
impl ConstantModulus for MOD998_244_353 {
    const VALUE: usize = 998_244_353;
}

impl Modulus for MOD998_244_353 {
    fn value() -> usize { Self::VALUE }
}

impl IsPrime for MOD998_244_353 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MOD1_000_000_007;
impl ConstantModulus for MOD1_000_000_007 {
    const VALUE: usize = 1_000_000_007;
}

impl Modulus for MOD1_000_000_007 {
    fn value() -> usize { Self::VALUE }
}

impl IsPrime for MOD1_000_000_007 {}

pub type Modular998_244_353 = Modular<MOD998_244_353>;
pub type Modular1_000_000_007 = Modular<MOD1_000_000_007>;

#[cfg(test)]
mod tests {
    #[test]
    fn test_static() {
        type Mint = super::Modular998_244_353;
        let mut x = Mint::new(998_244_353);
        assert_eq!(x.value(), 0);
        x += Mint::new(10);
        assert_eq!(x.value(), 10);
        x -= Mint::new(9);
        assert_eq!(x.value(), 1);
        x /= Mint::new(2);
        assert_eq!(x.value(), 499122177);
        assert_eq!(x, x);
        assert_eq!(x * x, x * x);
    }
}
