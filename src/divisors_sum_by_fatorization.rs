use crate::divisors_sum_from_prime_factors::divisors_sum_from_prime_factors;

pub fn divisors_sum_by_factorization<F>(factorize: &F, n: u64) -> u64
where
    F: Fn(u64) -> Vec<(u64, u8)>,
{
    divisors_sum_from_prime_factors(&factorize(n))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::prime_factorize_trial_division::*;

        assert_eq!(
            divisors_sum_by_factorization(
                &prime_factorize_trial_division,
                24
            ),
            (1 + 2 + 4 + 8) * (1 + 3)
        );
    }
}
