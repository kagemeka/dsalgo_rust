/// compute g := \gcd(modulus, n),
/// and modular inverse of n/g in Z_{modulus/g}.
/// we convert given values i64 internally.
/// so it overflows if modulus > 2^63.
/// it's `trivial` that inverse of 0 is undefined, so if n = 0, it panics.
pub fn extgcd_modinv(modulus: u64, n: u64) -> (u64, u64) {
    assert!(0 < n && n < modulus);
    let (mut a, mut b) = (n as i64, modulus as i64);
    let (mut x00, mut x01) = (1, 0);
    while b != 0 {
        // (x00, x01) = (x01, x00 - a / b * x01);
        // (a, b) = (b, a % b);

        std::mem::swap(&mut x00, &mut x01);
        x01 -= a / b * x00;
        std::mem::swap(&mut a, &mut b);
        b %= a;
    }
    if x00 < 0 {
        x00 += modulus as i64 / a;
    }
    debug_assert!(0 <= x00 && x00 < modulus as i64 / a);
    (a as u64, x00 as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        // extgcd_modinv(10, 0); // runtime error.
        assert_eq!(extgcd_modinv(5, 2), (1, 3));
        assert_eq!(extgcd_modinv(18, 12), (6, 2));
        assert_eq!(extgcd_modinv(111, 30), (3, 26));
        // 111 / 3 = 37, 30 / 3 = 10, 10^{-1} \equiv 26 \mod 37
    }
}
