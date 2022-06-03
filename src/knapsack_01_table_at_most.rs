/// dp[i] := max sum of values such that their weights sum is `at most` i.
pub(crate) fn knapsack_01_table(
    value_weight_pairs: &[(u64, u64)],
    size: usize,
) -> Vec<u64> {
    let mut max_value = vec![0; size];
    for &(v, w) in value_weight_pairs {
        let w = w as usize;
        for i in (w..size).rev() {
            max_value[i] = std::cmp::max(
                max_value[i],
                max_value[i - w] + v,
            );
        }
    }
    max_value
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
