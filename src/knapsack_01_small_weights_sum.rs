use crate::knapsack_01::knapsack_01;

pub fn knapsack_01_small_weights_sum(
    value_weight_pairs: &[(u64, u64)],
    capacity: u64,
) -> u64 {
    knapsack_01(
        value_weight_pairs,
        std::cmp::min(
            value_weight_pairs.iter().map(|&(_, w)| w).sum::<u64>(),
            capacity,
        ),
    )
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
