use crate::divisors_count_from_prime_factors::*;

pub fn count_divisors_by_factorization<F>(factorize: &F, n: u64) -> u64
where
    F: Fn(u64) -> Vec<(u64, u8)>,
{
    divisors_count_from_prime_factors(&factorize(n))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::prime_factorize_trial_division::*;

        assert_eq!(
            count_divisors_by_factorization(
                &prime_factorize_trial_division,
                24
            ),
            4 * 2,
        );
    }
}
