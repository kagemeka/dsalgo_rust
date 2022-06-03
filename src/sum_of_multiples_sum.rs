/// \sum_{i=1}^{limit}{\sum_{i|j, j <= limit}{j}}
/// smart formula transformation with symmetric property.
/// O(\sqrt{n})
pub fn sum_of_multiples_sum(limit: u64) -> u64 {
    let mut s = 0;
    for i in 1..=limit {
        if i * i > limit {
            break;
        }
        let j = limit / i;
        s += i * i + i * (i + 1 + j) * (j - i);
    }
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            sum_of_multiples_sum(10000000),
            838627288460105
        );
    }
}
