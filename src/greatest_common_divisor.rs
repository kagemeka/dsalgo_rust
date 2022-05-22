pub fn gcd_recurse(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd_recurse(b, a % b) }
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
        // (a, b) = (b, a % b);
    }
    a
}

pub fn signed_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    if a < 0 {
        a *= -1;
    }
    a
}

pub fn gcd_reduce(values: &[u64]) -> u64 {
    values.iter().fold(0, |accum, &x| gcd(accum, x))
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
        assert_eq!(gcd_reduce(&[]), 0);
        assert_eq!(gcd_reduce(&[2, 8, 4]), 2);
    }
}
