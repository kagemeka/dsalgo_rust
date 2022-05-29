use crate::{find_prime_numbers::find_prime_numbers, floor_sqrt::floor_sqrt};

pub struct RangeSieveOfEratosthenes {
    primes: Vec<u64>,
    less_than: u64,
}

impl RangeSieveOfEratosthenes {
    pub fn new(less_than: u64) -> Self {
        Self {
            primes: find_prime_numbers(floor_sqrt(less_than) + 1),
            less_than,
        }
    }

    /// find prime numbers in [lo, hi).
    /// time: O((hi - lo)\log{\log{less_than}})
    /// space: O(hi - lo)
    pub fn find_prime_numbers(&self, mut lo: u64, hi: u64) -> Vec<u64> {
        assert!(lo <= hi && hi <= self.less_than);
        if hi <= 2 {
            return vec![];
        }
        if lo < 2 {
            lo = 2;
        }

        let lo = lo as usize;
        let hi = hi as usize;
        let size = hi - lo;
        let mut is_prime = vec![true; size];
        for i in (lo & 1..size).step_by(2) {
            is_prime[i] = false;
        }
        if lo == 2 {
            is_prime[0] = true;
        }
        for &p in self.primes.iter().skip(1) {
            let i = p as usize;
            if i * i >= hi {
                break;
            }
            let mut from = (lo + i - 1) / i * i;
            if from & 1 == 0 {
                from += i;
            }
            for j in (std::cmp::max(from, i * i)..hi).step_by(i << 1) {
                is_prime[j - lo] = false;
            }
        }
        is_prime
            .into_iter()
            .enumerate()
            .filter_map(
                |(i, is_prime)| {
                    if is_prime { Some((i + lo) as u64) } else { None }
                },
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let sieve = RangeSieveOfEratosthenes::new(1 << 10);
        assert_eq!(
            sieve.find_prime_numbers(100, 500),
            vec![
                101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157,
                163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227,
                229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283,
                293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367,
                373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439,
                443, 449, 457, 461, 463, 467, 479, 487, 491, 499,
            ],
        );
    }
}
