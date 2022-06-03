use crate::divisors_from_prime_factors::divisors_from_prime_factors;

pub fn structive_find_divisors<F>(prime_factorize: &F, n: u64) -> Vec<u64>
where
    F: Fn(u64) -> Vec<(u64, u8)>,
{
    divisors_from_prime_factors(&prime_factorize(n))
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
