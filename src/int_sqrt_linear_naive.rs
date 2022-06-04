pub fn int_sqrt_linear_naive(n: u64) -> u64 {
    let mut x = 0;
    while x * x <= n {
        x += 1;
    }
    x - 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let res = (0..10).map(|x| int_sqrt_linear_naive(x)).collect::<Vec<_>>();
        assert_eq!(
            res,
            [0, 1, 1, 1, 2, 2, 2, 2, 2, 3]
        );
    }
}
