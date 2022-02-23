pub fn least_prime_factor(n: usize) -> Vec<usize> {
    assert!(n >= 2);
    let mut s: Vec<usize> = (0..n).collect();
    s[1] = 0;
    let mut i = 0;
    while i * i < n - 1 {
        i += 1;
        if s[i as usize] != i {
            continue;
        }
        for j in (i * i..n).step_by(i as usize) {
            if s[j as usize] == j {
                s[j as usize] = i;
            }
        }
    }
    s
}

pub fn greatest_prime_factor(n: usize) -> Vec<usize> {
    assert!(n >= 2);
    let mut s: Vec<usize> = (0..n).collect();
    s[1] = 0;
    let mut i = 0;
    while i < n - 1 {
        i += 1;
        if s[i as usize] != i {
            continue;
        }
        for j in (i * 2..n).step_by(i as usize) {
            s[j as usize] = i;
        }
    }
    s
}

pub fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let lpf = least_prime_factor(n);
    (0..n).map(|i| i >= 2 && i == lpf[i as usize]).collect()
}

pub fn find_prime_numbers(n: usize) -> Vec<usize> {
    let is_prime = sieve_of_eratosthenes(n);
    (0..n).filter(|i| is_prime[*i as usize]).collect()
}

pub fn prime_factorize(
    mut n: usize,
) -> std::collections::BTreeMap<usize, usize> {
    let mut cnt = std::collections::BTreeMap::new();
    let mut i = 1;
    while i * i < n {
        i += 1;
        if n % i != 0 {
            continue;
        }
        while n % i == 0 {
            n /= i;
            *cnt.entry(i).or_insert(0usize) += 1;
        }
    }
    if n > 1 {
        cnt.insert(n, 1);
    }
    cnt
}

pub struct PrimeFactorizeLPF {
    lpf: Vec<usize>,
}

impl PrimeFactorizeLPF {
    pub fn new(n: usize) -> Self {
        PrimeFactorizeLPF {
            lpf: least_prime_factor(n),
        }
    }

    pub fn factorize(
        &self,
        mut n: usize,
    ) -> std::collections::BTreeMap<usize, usize> {
        let mut cnt = std::collections::BTreeMap::new();
        while n > 1 {
            let p = self.lpf[n] as usize;
            n /= p;
            *cnt.entry(p).or_insert(0usize) += 1;
        }
        cnt
    }
}

pub fn count_prime_factors(n: usize) -> Vec<usize> {
    let mut cnt = vec![0; n as usize];
    for p in find_prime_numbers(n).into_iter().map(|x| x as usize) {
        for i in (p..n).step_by(p) {
            cnt[i] += 1;
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_prime_factor() {
        let lpf = least_prime_factor(1 << 10);
        assert_eq!(
            lpf.into_iter().take(10).collect::<Vec<_>>(),
            vec![0, 0, 2, 3, 2, 5, 2, 7, 2, 3],
        );
    }

    #[test]
    fn test_greatest_prime_factor() {
        let gpf = greatest_prime_factor(100);
        assert_eq!(gpf[51], 17);
        assert_eq!(
            gpf.into_iter().take(10).collect::<Vec<_>>(),
            vec![0, 0, 2, 3, 2, 5, 3, 7, 2, 3],
        );
    }

    #[test]
    fn test_sieve_of_eratosthenes() {
        let s = sieve_of_eratosthenes(1 << 10);
        assert_eq!(
            s.into_iter().take(10).collect::<Vec<_>>(),
            vec![
                false, false, true, true, false, true, false, true, false,
                false
            ],
        );
    }

    #[test]
    fn test_find_prime_numbers() {
        let primes = find_prime_numbers(1 << 10);
        assert_eq!(
            primes.into_iter().take(10).collect::<Vec<_>>(),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29
            ],
        );
    }

    #[test]
    fn test_prime_factorize_lpf() {
        let lpf = PrimeFactorizeLPF::new(1 << 10);
        use std::{
            array::IntoIter,
            collections::BTreeMap,
            iter::FromIterator,
        };
        assert_eq!(
            lpf.factorize(105),
            BTreeMap::from_iter(IntoIter::new([(3, 1), (5, 1), (7, 1)])),
        );
    }

    #[test]
    fn test_prime_factorize() {
        use std::{
            array::IntoIter,
            collections::BTreeMap,
            iter::FromIterator,
        };
        assert_eq!(
            prime_factorize(105),
            BTreeMap::from_iter(IntoIter::new([(3, 1), (5, 1), (7, 1)])),
        );
    }

    #[test]
    fn test_count_prime_factors() {
        let cnt = count_prime_factors(16);
        assert_eq!(
            cnt,
            vec![
                0, 0, 1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 2, 1, 2, 2
            ]
        );
    }
}
