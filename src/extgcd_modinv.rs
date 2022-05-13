/// compute g := \gcd(modulus, n),
/// and modular inverse of n/g in Z_{modulus/g}.
/// interface is i64 but u64 because it overflows if modulus > 2^63.
/// by converting it to i64 internally.
pub fn extgcd_modinv(modulus: i64, n: i64) -> (i64, Option<i64>) {
    assert!(modulus > 1 && 0 <= n && n < modulus);
    if n == 0 {
        return (modulus, None);
    }
    let (mut a, mut b) = (n, modulus);
    let (mut x00, mut x01) = (1, 0);
    while b != 0 {
        // this syntax is not supported on AtCoder yet.
        // (x00, x01) = (x01, x00 - a / b * x01);
        // (a, b) = (b, a % b);

        // old writing version.
        std::mem::swap(&mut x00, &mut x01);
        x01 -= a / b * x00;
        std::mem::swap(&mut a, &mut b);
        b %= a;
    }
    if x00 < 0 {
        x00 += modulus / a;
    }
    assert!(0 <= x00 && x00 < modulus / a);
    (a, Some(x00))
}
