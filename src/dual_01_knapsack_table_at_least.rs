/// dp[i] := min sum of weights such that their values sum is `at least` i.
pub(crate) fn dual_knapsack_01_table(
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
        for i in (1..size).rev() {
            if min_weight[i].is_none() {
                continue;
            }
            if min_weight[i - 1].is_none() || min_weight[i] < min_weight[i - 1]
            {
                min_weight[i - 1] = min_weight[i];
            }
        }
    }
    min_weight
}

/// dp[i] := min sum of weights such that their values sum is `at least` i.
#[allow(dead_code)]
pub(crate) fn dual_knapsack_01_table_fast(
    value_weight_pairs: &[(u64, u64)],
    size: usize,
) -> Vec<Option<u64>> {
    let mut min_weight = vec![None; size];
    min_weight[0] = Some(0);
    for &(v, w) in value_weight_pairs {
        let v = v as usize;
        for i in (0..size).rev() {
            if min_weight[i].is_none() {
                continue;
            }
            let nw = Some(min_weight[i].unwrap() + w);
            let j = std::cmp::min(i + v, size - 1);
            if min_weight[j].is_none() || nw < min_weight[j] {
                min_weight[j] = nw;
            }
        }
    }
    for i in (1..size).rev() {
        if min_weight[i].is_none() {
            continue;
        }
        if min_weight[i - 1].is_none() || min_weight[i] < min_weight[i - 1] {
            min_weight[i - 1] = min_weight[i];
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
