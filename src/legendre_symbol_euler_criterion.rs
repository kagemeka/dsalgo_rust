use crate::euler_criterion::euler_criterion;

/// prime modulus p and 0 <= a < p
/// if 0 < a, it's trivial gcd(a, p) = 1.
pub fn legendre_symbol_euler_criterion(p: u64, a: u64) -> i8 {
    match a {
        0 => 0,
        _ => match euler_criterion(p, a) {
            1 => 1,
            x if x == p - 1 => -1,
            _ => panic!("p cannot be prime."),
        },
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
