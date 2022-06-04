use crate::upper_bound_on_sequence::upper_bound;

pub fn knapsack_01_meet_in_the_middle(
    value_weight_pairs: &[(u64, u64)],
    capacity: u64,
) -> u64 {
    fn enumerate_bits_brute_force(items: &[(u64, u64)]) -> Vec<(u64, u64)> {
        let n = items.len();
        let mut cand = vec![];
        for s in 0..1 << n {
            let mut value = 0;
            let mut weight = 0;
            for i in 0..n {
                if s >> i & 1 == 0 {
                    continue;
                }
                let (v, w) = items[i];
                value += v;
                weight += w;
            }
            cand.push((value, weight));
        }
        cand.sort_by_key(|&(_, w)| w);
        for i in 0..(1 << n) - 1 {
            cand[i + 1].0 = std::cmp::max(cand[i].0, cand[i + 1].0);
        }
        cand
    }
    let n = value_weight_pairs.len();
    let a = enumerate_bits_brute_force(&value_weight_pairs[..n >> 1]);
    let b = enumerate_bits_brute_force(&value_weight_pairs[n >> 1..]);
    let b_weights = b.iter().map(|&(_, w)| w).collect::<Vec<_>>();
    let mut max_value = 0;
    for &(v, w) in a.iter() {
        if w > capacity {
            break;
        }
        let i = upper_bound(&b_weights, &(capacity - w));
        debug_assert!(i > 0);
        max_value = std::cmp::max(max_value, v + b[i - 1].0);
    }
    max_value
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
