use crate::{
    floor_sqrt::floor_sqrt,
    range_sieve_of_eratosthenes::RangeSieveOfEratosthenes,
};
pub struct SieveOfEratosthenesLowMemoryPrimeGenerator {
    iter: std::vec::IntoIter<u64>,
    range_sieve: RangeSieveOfEratosthenes,
    ranges: std::vec::IntoIter<(u64, u64)>,
}

impl SieveOfEratosthenesLowMemoryPrimeGenerator {
    /// [lo, hi)
    pub fn new(mut lo: u64, mut hi: u64) -> Self {
        if lo < 2 {
            lo = 2;
        }
        if hi < 2 {
            hi = 2;
        }
        let mut ranges = vec![];
        let range_size = (floor_sqrt(hi) as usize) << 3; // 2 or 3?
        // because range sieve has only odd numbers internally,
        // the size is sqrt / 2.
        // so we can check more than twice the range at once.
        // four times is best in test.
        for i in (lo..hi).step_by(range_size) {
            ranges.push((
                i,
                std::cmp::min(hi, i + range_size as u64),
            ));
        }

        Self {
            iter: vec![].into_iter(),
            range_sieve: RangeSieveOfEratosthenes::new(hi as u64),
            ranges: ranges.into_iter(),
        }
    }
}

impl Iterator for SieveOfEratosthenesLowMemoryPrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.iter.next() {
            return Some(p);
        }
        while let Some((lo, hi)) = self.ranges.next() {
            self.iter = self.range_sieve.find_prime_numbers(lo, hi).into_iter();
            if let Some(p) = self.iter.next() {
                return Some(p);
            }
        }
        None
    }
}

pub fn prime_generator(
    lo: u64,
    hi: u64,
) -> SieveOfEratosthenesLowMemoryPrimeGenerator {
    SieveOfEratosthenesLowMemoryPrimeGenerator::new(lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut iter = prime_generator(20, 30);
        assert_eq!(iter.next(), Some(23));
        assert_eq!(iter.next(), Some(29));
        assert_eq!(iter.next(), None);
    }
}
