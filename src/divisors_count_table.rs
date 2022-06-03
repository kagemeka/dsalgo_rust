pub fn divisors_count_table(size: usize) -> Vec<u64> {
    let mut cnt = vec![0; size];
    for i in (1..size).rev() {
        for j in (i..size).step_by(i) {
            cnt[j] += 1;
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            divisors_count_table(10),
            [0, 1, 2, 2, 3, 2, 4, 2, 4, 3],
        );
    }
}
