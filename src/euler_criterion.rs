use crate::power_semigroup::pow_semigroup;

/// prime modulus p and a != 0 (mod p)
pub fn euler_criterion(p: u64, a: u64) -> u64 {
    assert!(2 < p && p & 1 == 1 && 0 < a && a < p);
    // TODO: assert p is prime if O(\log{p}) possible
    assert!(a % p != 0);
    let res = pow_semigroup(
        &|x, y| (x as u128 * y as u128 % p as u128) as u64,
        a,
        (p - 1) >> 1,
    );
    debug_assert!(res == 1 || res == p - 1);
    res
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
