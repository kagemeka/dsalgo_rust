use crate::subset_sum_min_count::subset_sum_min_count_table;

pub fn is_achievable_subset_sum_at_most_k(
    values: &[u64],
    k: u64,
    target: u64,
) -> bool {
    let target = target as usize;
    if let Some(c) = subset_sum_min_count_table(values, target + 1)[target] {
        c <= k
    } else {
        false
    }
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
