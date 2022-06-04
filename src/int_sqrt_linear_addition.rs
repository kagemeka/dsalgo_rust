pub fn int_sqrt_linear_addition(n: u64) -> u64 {
    let mut x = 0;
    let mut x2 = 0; // x^2
    let mut delta = 1; // x2 + delta = (x + 1)^2
    while x2 <= n {
        x += 1;
        x2 += delta;
        delta += 2;
    }
    x - 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::int_sqrt_linear_naive::int_sqrt_linear_naive;
        for i in 0..1000 {
            assert_eq!(
                int_sqrt_linear_addition(i),
                int_sqrt_linear_naive(i)
            );
        }
    }
}
