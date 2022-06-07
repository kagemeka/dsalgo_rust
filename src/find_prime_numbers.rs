use crate::sieve_of_eratosthenes::sieve_of_eratosthenes;

pub fn find_prime_numbers(less_than: u32) -> Vec<u32> {
    sieve_of_eratosthenes(less_than as usize)
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
