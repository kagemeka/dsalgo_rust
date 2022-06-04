/// reference
/// https://en.wikipedia.org/wiki/Integer_square_root
pub fn int_sqrt_linear_subtraction(n: u64) -> u64 {
    let mut a = 5 * n;
    let mut b = 5;
    while a >= b {
        a -= b;
        b += 10;
    }
    b / 10
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::int_sqrt_linear_naive::int_sqrt_linear_naive;
        for i in 0..1000 {
            assert_eq!(
                int_sqrt_linear_subtraction(i),
                int_sqrt_linear_naive(i)
            );
        }
    }
}
