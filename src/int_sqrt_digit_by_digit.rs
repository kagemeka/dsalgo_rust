pub fn int_sqrt_digit_by_digit(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let x = int_sqrt_digit_by_digit(n >> 2) << 1;
    if (x + 1).pow(2) <= n { x + 1 } else { x }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::int_sqrt_linear_naive::int_sqrt_linear_naive;
        for i in 0..1000 {
            assert_eq!(
                int_sqrt_digit_by_digit(i),
                int_sqrt_linear_naive(i)
            );
        }
    }
}
