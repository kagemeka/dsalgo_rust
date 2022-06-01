/// gcd(0, 0) is not unique -> panic.
pub fn signed_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    assert_ne!(a, 0);
    a.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(signed_gcd(100, -3), 1);
        assert_eq!(signed_gcd(-1, 0), 1);
    }
}
