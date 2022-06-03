/// dp[i] := max sum of values such that their weights sum is `just` i.
#[allow(dead_code)]
pub(crate) fn knapsack_01_table_just(
    value_weight_pairs: &[(u64, u64)],
    size: usize,
) -> Vec<Option<u64>> {
    let mut max_value = vec![None; size];
    max_value[0] = Some(0);
    for &(v, w) in value_weight_pairs {
        let w = w as usize;
        for i in (w..size).rev() {
            if max_value[i - w].is_none() {
                continue;
            }
            let nv = Some(max_value[i - w].unwrap() + v);
            if max_value[i].is_none() || nv > max_value[i] {
                max_value[i] = nv;
            }
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
