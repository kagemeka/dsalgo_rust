use crate::is_prime_naive::is_prime_naive;

// TODO: use miller rabin
pub fn next_prime(mut n: u64) -> u64 {
    loop {
        n += 1;
        if is_prime_naive(n) {
            break;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(next_prime(53), 59);
    }
}
