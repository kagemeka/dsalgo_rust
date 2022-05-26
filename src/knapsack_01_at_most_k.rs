/// return max sum of values of at most k items
/// whose sum of weights is at most w.
pub fn knapsack_01_at_most_k(
    value_weight_pairs: &[(u64, u64)],
    k: u64,
    capacity: u64,
) -> u64 {
    let n = value_weight_pairs.len();
    let k = k as usize;
    let c = capacity as usize;
    assert!(k <= n);
    let mut max_value = vec![vec![0; c + 1]; k + 1];
    for &(v, w) in value_weight_pairs {
        let w = w as usize;
        for i in (0..k).rev() {
            for j in w..=c {
                max_value[i + 1][j] = std::cmp::max(
                    max_value[i + 1][j],
                    max_value[i][j - w] + v,
                );
            }
        }
    }
    max_value[k][c]
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
