use crate::sum_of_multiples_count_range::sum_of_multiples_count_range;

/// [1, n]
pub fn sum_of_divisors_count(n: u64) -> u64 {
    sum_of_multiples_count_range(n, 1, n)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(sum_of_divisors_count(4), 8);
    }
}
