use crate::unbounded_knapsack_table_at_most::unbounded_knapsack_table;

/// max sum of values such that their weights sum is `at most` capacity.
pub fn unbounded_knapsack(
    value_weight_pairs: &[(u64, u64)],
    capacity: u64,
) -> u64 {
    let c = capacity as usize;
    unbounded_knapsack_table(value_weight_pairs, c + 1)[c]
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
