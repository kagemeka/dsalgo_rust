pub fn divisors_sum_table(size: usize) -> Vec<u64> {
    let mut s = vec![0; size];
    for i in (1..size).rev() {
        for j in (i..size).step_by(i) {
            s[j] += i as u64;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            divisors_sum_table(10),
            [0, 1, 3, 4, 7, 6, 12, 8, 15, 13],
        );
    }
}
