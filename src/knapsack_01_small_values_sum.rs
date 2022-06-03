use crate::dual_01_knapsack_table_just::dual_knapsack_01_table_just;

/// max sum of values such that their weights sum is `at most` capacity.
pub fn knapsack_01_small_values_sum(
    value_weight_pairs: &[(u64, u64)],
    capacity: u64,
) -> u64 {
    dual_knapsack_01_table_just(
        value_weight_pairs,
        value_weight_pairs.iter().map(|&(v, _)| v).sum::<u64>() as usize + 1,
    )
    .into_iter()
    .enumerate()
    .filter_map(|(v, min_w)| {
        if let Some(w) = min_w {
            if w <= capacity { Some(v as u64) } else { None }
        } else {
            None
        }
    })
    .max()
    .unwrap()
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
