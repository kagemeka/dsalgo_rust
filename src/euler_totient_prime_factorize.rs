/// number of 1 <= k <= n, gcd(k, n) = 1.
pub fn euler_totient_prime_factorize<F>(factorize: &F, mut n: u64) -> u64
where
    F: Fn(u64) -> Vec<(u64, u8)>,
{
    for (p, _) in factorize(n as u64) {
        debug_assert_eq!(n % p, 0);
        n -= n / p;
    }
    n
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::prime_factorize_trial_division::*;
        // https://oeis.org/A000010
        const PHI: &'static [u64] = &[
            1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12,
            10, 22, 8, 20, 12, 18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18,
            24, 16, 40, 12, 42, 20, 24, 22, 46, 16, 42, 20, 32, 24, 52, 18, 40,
            24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66, 32, 44,
        ];
        for i in 0..PHI.len() {
            assert_eq!(
                euler_totient_prime_factorize(
                    &prime_factorize_trial_division,
                    (i + 1) as u64
                ),
                PHI[i]
            );
        }
    }
}
