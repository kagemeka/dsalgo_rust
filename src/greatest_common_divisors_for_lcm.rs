use crate::find_divisors_naive::find_divisors_naive;

/// \lcm(a_0, ...a_{n-1}) = `lcm`
pub fn gcds_for_lcm(n: usize, lcm: u64) -> Vec<u64> {
    match n {
        0 => {
            assert!(lcm == 1);
            vec![0]
        },
        1 => {
            vec![lcm]
        },
        _ => {
            assert!(lcm > 0); // n > 1 and lcm = 0 -> undefined.
            find_divisors_naive(lcm)
        },
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
