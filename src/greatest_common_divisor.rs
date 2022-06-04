/// gcd(0, 0) is infinity -> panic.
/// user can redefine gcd(0, 0) := 0 outside of this function.
/// but strictlly, we should not return gcd(0, 0).
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    assert_ne!(a, 0);
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(gcd(10, 5), 5);
        assert_eq!(gcd(0, 10), 10);
    }
}
