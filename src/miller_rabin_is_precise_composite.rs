use crate::{
    montgomery_modular_multiplication_64::MontgomeryMultiplication64,
    power_semigroup::pow_semigroup,
};

pub(crate) fn is_precise_composite(base: u64, n: u64) -> bool {
    debug_assert!(n > 2 && n & 1 == 1 && 0 < base && base < n);
    let (mut s, mut d) = (0, n - 1);
    // n - 1 = 2^s*d
    while d & 1 == 0 {
        s += 1;
        d >>= 1;
    }
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
