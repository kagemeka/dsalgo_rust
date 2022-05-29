pub fn gcd_recurse(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd_recurse(b, a % b) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(gcd_recurse(10, 5), 5);
        assert_eq!(gcd_recurse(0, 10), 10);
        assert_eq!(gcd_recurse(0, 0), 0);
    }
}
