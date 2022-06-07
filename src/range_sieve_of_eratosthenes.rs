use crate::{find_prime_numbers::find_prime_numbers, floor_sqrt::floor_sqrt};
pub struct RangeSieveOfEratosthenes {
    primes: Vec<u64>,
    less_than: u64,
}

impl RangeSieveOfEratosthenes {
    pub fn new(less_than: u64) -> Self {
        Self {
            primes: find_prime_numbers(floor_sqrt(less_than) as u32 + 1)
                .into_iter()
                .map(|p| p as u64)
                .collect(),
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
        debug_assert!(2 <= lo && lo < hi);
        let mut lo = lo as usize;
        let mut hi = hi as usize;
        let mut size = hi - lo;
        let mut res = Vec::with_capacity(size >> 1);
        if lo & 1 == 0 {
            if lo == 2 {
                res.push(2);
            }
            lo += 1;
            size -= 1;
        }
        if size == 0 {
            return res;
        }
        if hi & 1 == 0 {
            hi += 1;
            size += 1;
        }
        debug_assert!(size & 1 == 0);
        // initially, only odd numbers are in sieve.
        // be careful of indices.
        let mut is_prime = vec![true; size >> 1];
        let size2 = size >> 1;
        for &p in self.primes.iter().skip(1) {
            let i = p as usize;
            let mut from = i * i;
            if from >= hi {
                break;
            }
            from = std::cmp::max(from, (lo + i - 1) / i * i);
            if from & 1 == 0 {
                from += i;
            }
            debug_assert!(from & 1 == 1);
            for j in ((from - lo) >> 1..size2).step_by(i) {
                is_prime[j] = false;
            }
        }
        res.extend(
            is_prime.into_iter().enumerate().filter_map(|(i, is_prime)| {
                if is_prime { Some((lo + (i << 1)) as u64) } else { None }
            }),
        );
        res
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
