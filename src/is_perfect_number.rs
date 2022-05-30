use crate::find_divisors_naive::find_divisors_naive;

// TODO: optimize divisor finding algorithm
pub fn is_perfect_number(n: u64) -> bool {
    find_divisors_naive(n).iter().sum::<u64>() == n << 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::perfect_numbers::PERFECT_NUMBERS;
        for &n in PERFECT_NUMBERS[..6].iter() {
            assert!(is_perfect_number(n));
        }
    }
}
