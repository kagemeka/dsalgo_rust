use crate::legendre_function::legendre_function;

/// trailing zeros in decimal.
pub fn count_factorial_trailing_zeros(n: u64) -> u64 { legendre_function(n, 5) }

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
