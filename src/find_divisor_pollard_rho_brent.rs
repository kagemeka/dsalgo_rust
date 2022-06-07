use crate::{
    abs_diff::abs_diff,
    bit_length::bit_length,
    greatest_common_divisor::gcd,
    montgomery_modular_multiplication_64::MontgomeryMultiplication64,
};

/// reference
/// - ja wiki
/// using brent's cycle detection algorithm to find a divisor.
/// faster than floyd's method.
pub fn find_divisor_pollard_rho_brent<F>(
    n: u64,
    prng_next: &F,
) -> Result<u64, &'static str>
where
    F: Fn(u64) -> u64,
{
    let multiplier = MontgomeryMultiplication64::new(n);
    let m = 1 << (bit_length(n) >> 3); // any other value?
    let (mut y, mut r, mut q, mut g) = (2, 1, 1, 1);
    let (mut x, mut ys) = (0, 0); // to be updated lazily.
    while g == 1 {
        x = y;
        for _ in 0..r {
            y = prng_next(y);
        }
        let mut k = 0;
        while k < r && g == 1 {
            ys = y;
            for _ in 0..std::cmp::min(m, r - k) {
                y = prng_next(y);
                q = multiplier.mul(q, abs_diff(x, y));
            }
            g = gcd(q, n);
            k += m;
        }
        r <<= 1;
    }
    if g == n {
        g = 1;
        while g == 1 {
            ys = prng_next(ys);
            g = gcd(abs_diff(x, ys), n);
        }
    }
    if g == n { Err("failed to find a divisor") } else { Ok(g) }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
