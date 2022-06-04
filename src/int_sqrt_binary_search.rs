pub fn int_sqrt_binary_search(n: u64) -> u64 {
    let mut lo = 0;
    let mut hi = 1 << 32;
    while hi - lo > 1 {
        let x = (lo + hi) >> 1;
        if x * x <= n {
            lo = x;
        } else {
            hi = x;
        }
    }
    lo
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::int_sqrt_linear_naive::int_sqrt_linear_naive;
        for i in 0..1000 {
            assert_eq!(
                int_sqrt_binary_search(i),
                int_sqrt_linear_naive(i)
            );
        }
    }
}
