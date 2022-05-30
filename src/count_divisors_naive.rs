use crate::find_divisors_naive::find_divisors_naive;

pub fn count_divisors_naive(n: u64) -> u32 {
    find_divisors_naive(n).len() as u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(count_divisors_naive(0), 0);
        assert_eq!(count_divisors_naive(1), 1);
        assert_eq!(count_divisors_naive(2), 2);
        assert_eq!(count_divisors_naive(19), 2);
        assert_eq!(count_divisors_naive(105), 8); // 3, 5, 7 -> 2^3
    }
}
