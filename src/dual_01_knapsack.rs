use crate::dual_01_knapsack_table_at_least::dual_knapsack_01_table;

pub fn dual_knapsack_01(
    value_weight_pairs: &[(u64, u64)],
    target_value: u64,
) -> Result<u64, &'static str> {
    let s = value_weight_pairs.iter().map(|&(v, _)| v).sum::<u64>();
    let s = s as usize;
    let t = target_value as usize;
    if s < t {
        return Err("sum of values cannot achieve target value");
    }
    Ok(dual_knapsack_01_table(value_weight_pairs, t + 1)[t].unwrap())
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
