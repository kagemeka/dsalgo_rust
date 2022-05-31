use crate::{
    find_prime_numbers::find_prime_numbers,
    legendre_function::legendre_function,
};

pub fn prime_factorize_factorial_legendre(n: u64) -> Vec<(u64, u64)> {
    find_prime_numbers(n + 1)
        .into_iter()
        .map(|p| (p, legendre_function(n, p)))
        .collect()
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
