pub fn sieve_of_eratosthenes(sieve_size: usize) -> Vec<u32> {
    let mut primes = Vec::with_capacity(sieve_size >> 1);
    if sieve_size > 2 {
        primes.push(2);
    }
    let mut is_prime = vec![true; sieve_size >> 1];
    for i in (3..sieve_size).step_by(2) {
        if !is_prime[i >> 1] {
            continue;
        }
        primes.push(i as u32);
        for j in (i * i >> 1..sieve_size >> 1).step_by(i) {
            is_prime[j] = false;
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            sieve_of_eratosthenes(20),
            vec![2, 3, 5, 7, 11, 13, 17, 19],
        )
    }
}
