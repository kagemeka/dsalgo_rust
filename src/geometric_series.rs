use crate::power_semigroup::pow_semigroup;

/// \sum_{i=0}^{n}a_0r^i
pub fn geometric_series(a0: i64, r: i64, n: usize) -> i64 {
    if r == 0 {
        return a0 as i64;
    }
    (1 - pow_semigroup(&|x, y| x * y, r, n as u64 + 1)) / (1 - r) * a0
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(geometric_series(3, 4, 3), 255);
        assert_eq!(geometric_series(3, -1, 3), 0);
        assert_eq!(geometric_series(3, 0, 3), 3);
    }
}
