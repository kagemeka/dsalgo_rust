use crate::geometric_series::geometric_series;

pub fn divisors_sum_from_prime_factors(prime_factors: &[(u64, u8)]) -> u64 {
    let mut sum = 1;
    for &(p, e) in prime_factors {
        sum *= geometric_series(1, p as i64, e as usize) as u64;
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::prime_factorize_trial_division::*;
        let prime_factors = prime_factorize_trial_division(24);
        assert_eq!(
            divisors_sum_from_prime_factors(&prime_factors),
            (1 + 2 + 4 + 8) * (1 + 3)
        );
    }
}
