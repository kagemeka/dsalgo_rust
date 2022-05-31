use crate::prime_factorize_trial_division::prime_factorize_trial_division;

// TODO: use faster factorization algorithm.
pub fn mobius_function(n: u64) -> i8 {
    assert!(n > 0);
    if n == 1 {
        return 1;
    }
    let mut u = 1;
    for (_, e) in prime_factorize_trial_division(n) {
        if e > 1 {
            return 0;
        }
        u *= -1;
    }
    u
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(mobius_function(1), 1);
        assert_eq!(mobius_function(2), -1);
        assert_eq!(mobius_function(4), 0);
    }
}
