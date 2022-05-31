pub fn sum_of_multiples(limit: u64, n: u64) -> u64 {
    let cnt = limit / n;
    (1 + cnt) * cnt / 2 * n
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            sum_of_multiples(10, 3),
            3 + 6 + 9
        );
    }
}
