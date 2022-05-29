use crate::sieve_of_eratosthenes::sieve_of_eratosthenes;

pub fn is_prime_table(size: usize) -> Vec<bool> { sieve_of_eratosthenes(size) }

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
