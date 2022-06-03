use crate::find_divisors_naive::find_divisors_naive;

/// 0 <= r < n
/// find all x such that n = qx + r
pub fn find_divisors_for_const_remainder(n: u64, r: u64) -> Vec<u64> {
    find_divisors_naive(n - r)
        .into_iter()
        .filter(|&d| d > r)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            find_divisors_for_const_remainder(1000, 30),
            [97, 194, 485, 970]
        );
    }
}
