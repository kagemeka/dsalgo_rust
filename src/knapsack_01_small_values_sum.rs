pub fn knapsack_01_small_values_sum(
    value_weight_pairs: &[(u64, u64)],
    capacity: u64,
) -> u64 {
    let s = value_weight_pairs.iter().map(|&(v, _)| v).sum::<u64>() as usize;
    let mut min_weight = vec![None; s + 1];
    min_weight[0] = Some(0);
    for &(v, w) in value_weight_pairs {
        let v = v as usize;
        for i in (v..=s).rev() {
            if min_weight[i - v].is_none() {
                continue;
            }
            let nw = Some(min_weight[i - v].unwrap() + w);
            if min_weight[i].is_none() || nw < min_weight[i] {
                min_weight[i] = nw;
            }
        }
    }
    min_weight
        .into_iter()
        .enumerate()
        .filter_map(|(i, w)| {
            if let Some(w) = w {
                if w <= capacity { Some(i as u64) } else { None }
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
