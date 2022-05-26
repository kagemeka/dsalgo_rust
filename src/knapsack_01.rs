pub fn knapsack_01(value_weight_pairs: &[(u64, u64)], capacity: u64) -> u64 {
    let c = capacity as usize;
    let mut max_value = vec![0; c + 1];
    for &(v, w) in value_weight_pairs {
        let w = w as usize;
        for i in (w..=c).rev() {
            max_value[i] = std::cmp::max(
                max_value[i],
                max_value[i - w] + v,
            );
        }
    }
    max_value[c]
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
