use crate::{
    make_sparse_histogram::make_sparse_histogram,
    prime_factorize_pollard_rho_flat::*,
};

pub fn prime_factorize_pollard_rho<F>(
    is_prime: &F,
    n: u64,
) -> Result<Vec<(u64, u8)>, &'static str>
where
    F: Fn(u64) -> bool,
{
    Ok(
        make_sparse_histogram(prime_factorize_flat_pollard_rho(is_prime, n)?)
            .into_iter()
            .map(|(p, c)| (p, c as u8))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::{
            primality_test_miller_rabin::*,
            prime_factorize_trial_division::*,
        };
        for i in 1..10000 {
            assert_eq!(
                prime_factorize_pollard_rho(&miller_rabin_test, i),
                Ok(prime_factorize_trial_division(
                    i
                )),
            );
        }
    }
}
