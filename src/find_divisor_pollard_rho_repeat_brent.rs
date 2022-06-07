use crate::{
    find_divisor_pollard_rho_brent::find_divisor_pollard_rho_brent,
    montgomery_modular_multiplication_64::MontgomeryMultiplication64,
};

pub fn find_divisor_pollard_rho_repeat(
    n: u64,
    epochs: u8,
) -> Result<u64, &'static str> {
    let multiplier = MontgomeryMultiplication64::new(n);
    for c in 1..=epochs {
        // this prng is referenced from wiki.
        let prng = |x: u64| (multiplier.mul(x, x) + c as u64) % n;
        // if let Ok(divisor) = find_divisor_pollard_rho_floyd(n, &prng) {
        if let Ok(divisor) = find_divisor_pollard_rho_brent(n, &prng) {
            return Ok(divisor);
        }
    }
    Err("no divisor found")
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
