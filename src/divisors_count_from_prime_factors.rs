pub fn divisors_count_from_prime_factors(prime_factors: &[(u64, u8)]) -> u64 {
    let mut count = 1;
    for &(_, e) in prime_factors {
        count *= e as u64 + 1;
    }
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::prime_factorize_trial_division::*;
        assert_eq!(
            divisors_count_from_prime_factors(
                &prime_factorize_trial_division(24)
            ),
            4 * 2,
        );
    }
}
