use crate::find_divisors_median_low::find_divisors_median_low;

pub fn min_pair_sum_const_prod(prod: u64) -> u64 {
    if prod == 0 {
        return 0;
    }
    let d = find_divisors_median_low(prod);
    d + prod / d
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(min_pair_sum_const_prod(1), 2);
        assert_eq!(
            min_pair_sum_const_prod(100),
            20
        );
        assert_eq!(
            min_pair_sum_const_prod(101),
            102
        );
    }
}
