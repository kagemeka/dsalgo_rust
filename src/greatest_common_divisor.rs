pub fn gcd_recurse(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd_recurse(b, a % b) }
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

pub fn signed_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a.abs()
}

/// internally, use fold but initial value (= gcd(empty)) is defined as 0.
pub fn gcd_reduce<I>(iter: I) -> u64
where
    I: Iterator<Item = u64>,
{
    iter.fold(0, |a, b| gcd(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(gcd_recurse(10, 5), 5);
        assert_eq!(gcd_recurse(0, 10), 10);
        assert_eq!(gcd_recurse(0, 0), 0);
        assert_eq!(gcd(10, 5), 5);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd_reduce([].into_iter()), 0);
        assert_eq!(
            gcd_reduce([2, 8, 4].into_iter()),
            2
        );
    }
}
