use crate::find_divisors_naive::find_divisors_naive;

/// find medians of divisors.
pub fn find_divisors_median_low(n: u64) -> u64 {
    assert!(n > 0);
    let divs = find_divisors_naive(n);
    divs[((divs.len() + 1) >> 1) - 1]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(find_divisors_median_low(1), 1);
        assert_eq!(find_divisors_median_low(2), 1);
        assert_eq!(find_divisors_median_low(3), 1);
        assert_eq!(find_divisors_median_low(4), 2);
    }
}
