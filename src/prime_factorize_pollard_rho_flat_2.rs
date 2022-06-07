use crate::find_prime_factor_pollard_rho_brent::find_prime_factor_pollard_rho;

pub fn prime_factorize_flat_pollard_rho_2<F>(
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
    for i in 3..100 {
        while n % i == 0 {
            n /= i;
            res.push(i);
        }
        if n == 1 {
            break;
        }
    }
    while n > 1 {
        let p = find_prime_factor_pollard_rho(is_prime, n, 1 << 7)?;
        while n % p == 0 {
            n /= p;
            res.push(p);
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
