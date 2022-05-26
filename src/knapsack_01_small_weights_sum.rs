use crate::knapsack_01::knapsack_01;

/// max sum of values such that their weights sum is `at most` capacity.
pub fn knapsack_01_small_weights_sum(
    value_weight_pairs: &[(u64, u64)],
    capacity: u64,
) -> u64 {
    knapsack_01(value_weight_pairs, capacity)
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
