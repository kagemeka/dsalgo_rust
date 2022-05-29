use crate::sieve_of_eratosthenes::sieve_of_eratosthenes;

pub fn find_prime_numbers(less_than: u64) -> Vec<u64> {
    sieve_of_eratosthenes(less_than as usize)
        .iter()
        .enumerate()
        .filter_map(
            |(i, &is_prime)| {
                if is_prime { Some(i as u64) } else { None }
            },
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            find_prime_numbers(50),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47
            ],
        );
    }
}
