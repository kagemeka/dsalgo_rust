/// k < 64, base := 2^k, K = base^2, x, y < K,
/// compute [x * y / K] without overflow.
pub fn karatsuba_quotient(k: u8, x: u128, y: u128) -> u128 {
    assert!(k < 64);
    let base = 1 << k;
    assert!(x >> (k << 1) == 0 && y >> (k << 1) == 0);
    let mask = base - 1;
    let (a1, a0) = (x >> k, x & mask);
    let (b1, b0) = (y >> k, y & mask);
    let c2 = a1 * b1;
    let c0 = a0 * b0;
    let c1 = (a1 + a0) * (b1 + b0) - c2 - c0;
    // max tmp value < (2base)^2 = 2^(2k + 2), 2k + 2 <= 128 -> k < 64
    c2 + ((c1 + (c0 >> k)) >> k)
    // = c2 + (((c1 << k) + c0) >> (k << 1)), but c1 << k can overflow
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
