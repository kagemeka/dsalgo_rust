use crate::is_prime_naive::is_prime_naive;

// TODO: accelerate wtih miller rabin.
/// for small n, use prime numbers list and precompute prev/next prime.
pub fn previous_prime_number(mut n: u64) -> u64 {
    assert!(n > 2);
    loop {
        n -= 1;
        if is_prime_naive(n) {
            break;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
