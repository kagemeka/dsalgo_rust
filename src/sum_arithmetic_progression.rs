pub fn sum_arithmetic_progression(a: i64, b: i64, count: u64) -> i64 {
    assert!(count > 0);
    if count == 1 {
        assert!(a == b);
    } else {
        assert!((b - a) % (count as i64 - 1) == 0);
    }
    (a + b) * count as i64 / 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            sum_arithmetic_progression(1, 10, 10),
            55
        );
        assert_eq!(
            sum_arithmetic_progression(1, 9, 5),
            25
        );
    }
}
