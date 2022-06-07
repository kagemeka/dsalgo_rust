use crate::find_divisor_pollard_rho_repeat_brent::*;

pub fn prime_factorize_flat_pollard_rho<F>(
    is_prime: &F,
    mut n: u64,
) -> Result<Vec<u64>, &'static str>
where
    F: Fn(u64) -> bool,
{
    let mut res = vec![];
    let ctz = n.trailing_zeros();
    res.extend(vec![2; ctz as usize]);
    n >>= ctz;
    if is_prime(n) {
        res.push(n);
        n = 1;
    }
    if n == 1 {
        return Ok(res);
    }
    let divisor = find_divisor_pollard_rho_repeat(n, 1 << 7);
    if divisor.is_err() {
        return Err("n is composite, and failed to factorize");
    }
    let d = divisor.unwrap();
    debug_assert!(1 < d && d < n);
    res.extend(prime_factorize_flat_pollard_rho(is_prime, d)?);
    res.extend(prime_factorize_flat_pollard_rho(is_prime, n / d)?);
    Ok(res)
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
