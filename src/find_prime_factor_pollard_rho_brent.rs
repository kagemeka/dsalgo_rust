use crate::{
    find_divisor_pollard_rho_brent::find_divisor_pollard_rho_brent,
    montgomery_modular_multiplication_64::*,
};

pub fn find_prime_factor_pollard_rho<F>(
    is_prime: &F,
    mut n: u64,
    epochs: u8,
) -> Result<u64, &'static str>
where
    F: Fn(u64) -> bool,
{
    let multiplier = MontgomeryMultiplication64::new(n);
    if is_prime(n) {
        return Ok(n);
    }
    for c in 1..=epochs {
        let prng = |x: u64| (multiplier.mul(x, x) + c as u64) % n;
        if let Ok(divisor) = find_divisor_pollard_rho_brent(n, &prng) {
            if is_prime(divisor) {
                return Ok(divisor);
            }
            n = divisor;
        }
    }
    Err("no divisor found")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
