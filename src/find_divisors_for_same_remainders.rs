use crate::find_divisors_naive::find_divisors_naive;

pub fn find_divisors_for_same_remainders(mut a: u64, mut b: u64) -> Vec<u64> {
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    find_divisors_naive(b - a)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            find_divisors_for_same_remainders(100, 30),
            [1, 2, 5, 7, 10, 14, 35, 70],
        );
    }
}
