use crate::{
    montgomery_modular_multiplication_64::MontgomeryMultiplication64,
    power_semigroup::pow_semigroup,
};

/// n might not be prime.
pub(crate) fn try_euler_criterion(n: u64, a: u64) -> Result<u64, &'static str> {
    assert!(2 < n && n & 1 == 1 && 0 < a && a < n);
    assert!(a % n != 0);
    let multiplier = MontgomeryMultiplication64::new(n);
    let res = pow_semigroup(
        &|x, y| multiplier.mul(x, y),
        a,
        (n - 1) >> 1,
    );
    if res == 1 || res == n - 1 {
        Ok(res)
    } else {
        Err("modulus n cannot be prime.")
    }
}

/// prime modulus p and a != 0 (mod p)
/// user must ensure that p is prime.
pub fn euler_criterion(p: u64, a: u64) -> u64 {
    // TODO: assert p is prime if O(\log{p}) possible
    try_euler_criterion(p, a).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(euler_criterion(5, 1), 1); // 1^2, 4^2
        assert_eq!(euler_criterion(5, 2), 4);
        assert_eq!(euler_criterion(5, 3), 4);
        assert_eq!(euler_criterion(5, 4), 1); // 2^2, 3^2
    }
}
