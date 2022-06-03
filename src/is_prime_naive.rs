use crate::find_divisors_naive::find_divisors_naive;

pub fn is_prime_naive(n: u64) -> bool { find_divisors_naive(n).len() == 2 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert!(!is_prime_naive(0));
        assert!(!is_prime_naive(1));
        assert!(is_prime_naive(2));
        assert!(!is_prime_naive(1000001));
        assert!(is_prime_naive(1000003));
    }
}
