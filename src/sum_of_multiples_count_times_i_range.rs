use crate::floor_sqrt::floor_sqrt;

/// \sum_{i=1}^n{i \cdot |multiples(i)|}
pub fn sum_of_multiples_count_times_i_range(
    limit: u64,
    lo: u64,
    mut hi: u64,
) -> u64 {
    // similar to sum of multiples sum.
    if limit < hi {
        hi = limit;
    }
    if hi < lo {
        return 0;
    }
    debug_assert!(lo <= hi && hi <= limit);
    let mut s = 0;
    let mut k: u64 = 0;
    for i in lo..=hi {
        if i * i > limit {
            k = i;
            break;
        }
        s += limit / i * i;
        // different from sum of multiples sum.
    }
    if k == 0 {
        debug_assert!(hi * hi <= limit);
        return s;
    }
    debug_assert!(k == floor_sqrt(limit) + 1);
    debug_assert!(lo <= k && k <= hi);
    for j in 1..=limit / k {
        let mut i_max = limit / j;
        debug_assert!((i_max + 1) * j > limit);
        debug_assert!(k <= i_max);
        if i_max > hi {
            i_max = hi;
        }
        s += (k + i_max) * (i_max - k + 1) / 2;
        // different from sum of multiples sum
    }
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            sum_of_multiples_count_times_i_range(4, 1, 4),
            1 * 4 + 2 * 2 + 3 * 1 + 4 * 1
        );
        assert_eq!(
            sum_of_multiples_count_times_i_range(10, 3, 5),
            3 * 3 + 4 * 2 + 5 * 2
        );
        println!(
            "{}",
            sum_of_multiples_count_times_i_range(10, 1, 2)
        );
        assert_eq!(
            sum_of_multiples_count_times_i_range(10, 1, 2),
            1 * 10 + 2 * 5
        );
    }
}
