pub fn int_sqrt_newton(n: u64) -> u64 {
    let mut x0 = n >> 1;
    if x0 == 0 {
        return n;
    }
    loop {
        let x1 = (x0 + n / x0) >> 1;
        if x1 >= x0 {
            break;
        }
        x0 = x1;
    }
    x0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::int_sqrt_linear_naive::int_sqrt_linear_naive;
        for i in 0..1000 {
            assert_eq!(
                int_sqrt_newton(i),
                int_sqrt_linear_naive(i)
            );
        }
    }
}
