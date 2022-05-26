use crate::subset_sum_at_most_k::is_achievable_subset_sum_at_most_k;

pub fn is_achievable_subset_sum(values: &[u64], target: u64) -> bool {
    is_achievable_subset_sum_at_most_k(
        values,
        values.len() as u64,
        target,
    )
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
