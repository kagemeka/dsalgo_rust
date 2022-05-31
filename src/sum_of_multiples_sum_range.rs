use crate::{floor_sqrt::floor_sqrt, sum_of_multiples::sum_of_multiples};

/// \sum_{i=lo}^{hi}{\sum_{i|j, j <= limit}{j}}
/// sqrt split teqnique
/// O(\sqrt{n})
pub fn sum_of_multiples_sum_range(limit: u64, lo: u64, mut hi: u64) -> u64 {
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
        s += sum_of_multiples(limit, i);
    }
    if k == 0 {
        debug_assert!(hi * hi <= limit);
        return s;
    }
    debug_assert!(k == floor_sqrt(limit) + 1);
    debug_assert!(lo <= k && k <= hi);
    // added for i=lo..k.
    // next, consider i=k..=hi.
    // for j=1..?, for i i * j <= limit, add sum(i * j) = j * sum(i)
    // because i >= k, it's enough to consider j as at most limit / k.
    for j in 1..=limit / k {
        let mut i_max = limit / j;
        debug_assert!((i_max + 1) * j > limit);
        debug_assert!(k <= i_max);
        if i_max > hi {
            i_max = hi;
        }
        s += (k + i_max) * (i_max - k + 1) / 2 * j;
    }
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            sum_of_multiples_sum_range(10000000, 1, 10000000),
            838627288460105
        );

        assert_eq!(
            sum_of_multiples_sum_range(10, 3, 5),
            18 + 12 + 15,
        );
    }
}
