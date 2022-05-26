pub fn subset_sum_min_count_table(
    values: &[u64],
    size: usize,
) -> Vec<Option<u64>> {
    let mut at_least = vec![None; size];
    at_least[0] = Some(0);
    for &v in values {
        let v = v as usize;
        for j in (v..size).rev() {
            if at_least[j - v].is_none() {
                continue;
            }
            let c = Some(at_least[j - v].unwrap() + 1);
            if at_least[j].is_none() || c < at_least[j] {
                at_least[j] = c;
            }
        }
    }
    at_least
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
