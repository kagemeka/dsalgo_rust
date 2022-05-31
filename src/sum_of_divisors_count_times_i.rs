use crate::sum_of_multiples_sum::sum_of_multiples_sum;

/// \sum_{i=1}^n{i \cdot |divisors(i)|}
pub fn sum_of_divisors_count_times_i(n: u64) -> u64 { sum_of_multiples_sum(n) }

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            sum_of_divisors_count_times_i(10000000),
            838627288460105
        );
    }
}
