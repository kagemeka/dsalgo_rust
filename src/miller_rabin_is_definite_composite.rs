use crate::{
    montgomery_modular_multiplication_64::MontgomeryMultiplication64,
    power_semigroup::pow_semigroup,
};

pub(crate) fn is_composite_miller_rabin(base: u64, n: u64) -> bool {
    assert!(n > 2 && n & 1 == 1 && 2 <= base && base < n - 1);
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    // n - 1 = 2^s*d
    let multiplier = MontgomeryMultiplication64::new(n);
    let mut x = pow_semigroup(
        &|x, y| multiplier.mul(x, y),
        base,
        d,
    );
    if x == 1 {
        return false;
    }
    for _ in 0..s {
        if x == n - 1 {
            return false;
        }
        x = multiplier.mul(x, x);
    }
    true
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
