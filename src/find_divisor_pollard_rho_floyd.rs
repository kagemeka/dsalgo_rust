use crate::{abs_diff::abs_diff, greatest_common_divisor::gcd};

/// n is not prime.
/// if prime, check in advance with primality test like miller-rabin.
/// return divisor of n. (1 < d < n)
pub fn find_divisor_pollard_rho_floyd<F>(
    n: u64,
    prng_next: &F,
) -> Result<u64, &'static str>
where
    F: Fn(u64) -> u64,
{
    assert!(n > 2);
    let (mut x, mut y, mut d) = (2, 2, 1);
    while d == 1 {
        x = prng_next(x);
        y = prng_next(prng_next(y));
        d = gcd(abs_diff(x, y), n);
    }
    if d == n { Err("No divisor found") } else { Ok(d) }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
