pub fn is_achievable_subset_sum_multiple_same_values(
    value_count_paris: &[(u64, u64)],
    target: u64,
) -> bool {
    let target = target as usize;
    let mut at_least = vec![None; target + 1];
    at_least[0] = Some(0);
    for &(v, at_most) in value_count_paris {
        assert!(v > 0);
        let v = v as usize;
        for j in 0..=target {
            if at_least[j].is_some() {
                at_least[j] = Some(0);
                continue;
            }
            if j < v {
                continue;
            }
            if let Some(c) = at_least[j - v] {
                if c < at_most {
                    at_least[j] = Some(c + 1);
                }
            }
        }
    }
    at_least[target].is_some()
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
