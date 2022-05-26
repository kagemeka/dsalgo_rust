use crate::knapsack_01_table_at_most::knapsack_01_table;

/// max sum of values such that their weights sum is `at most` capacity.
pub fn knapsack_01(value_weight_pairs: &[(u64, u64)], capacity: u64) -> u64 {
    if value_weight_pairs.iter().map(|&(_, w)| w).sum::<u64>() <= capacity {
        value_weight_pairs.iter().map(|&(v, _)| v).sum()
    } else {
        let c = capacity as usize;
        // knapsack_01_table_just(value_weight_pairs, c + 1)
        //     .iter()
        //     .filter_map(|&x| x)
        //     .max()
        //     .unwrap()
        knapsack_01_table(value_weight_pairs, c + 1)[c]
    }
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
