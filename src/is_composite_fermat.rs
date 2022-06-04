use crate::{
    greatest_common_divisor::gcd,
    montgomery_modular_multiplication_64::MontgomeryMultiplication64,
    power_semigroup::pow_semigroup,
};

pub fn is_composite_fermat(base: u64, n: u64) -> bool {
    assert!(
        n > 2 && n & 1 == 1 && 2 <= base && base < n - 1 && gcd(base, n) == 1
    );
    let multiplier = MontgomeryMultiplication64::new(n);
    pow_semigroup(
        &|x, y| multiplier.mul(x, y),
        base,
        n - 1,
    ) != 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
