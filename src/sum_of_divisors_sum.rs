use crate::sum_of_multiples_count_times_i_range::*;

pub fn sum_of_divisors_sum(n: u64) -> u64 {
    sum_of_multiples_count_times_i_range(n, 1, n)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(sum_of_divisors_sum(4), 15);
    }
}
