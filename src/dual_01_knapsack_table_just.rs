/// dp[i] := min sum of weights such that their values sum is `just` i.
pub(crate) fn dual_knapsack_01_table_just(
    value_weight_pairs: &[(u64, u64)],
    size: usize,
) -> Vec<Option<u64>> {
    let mut min_weight = vec![None; size];
    min_weight[0] = Some(0);
    for &(v, w) in value_weight_pairs {
        let v = v as usize;
        for i in (v..size).rev() {
            if min_weight[i - v].is_none() {
                continue;
            }
            let nw = Some(min_weight[i - v].unwrap() + w);
            if min_weight[i].is_none() || nw < min_weight[i] {
                min_weight[i] = nw;
            }
        }
    }
    min_weight
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
