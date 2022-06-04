use crate::int_sqrt_binary_search::int_sqrt_binary_search;

pub fn floor_sqrt(n: u64) -> u64 { int_sqrt_binary_search(n) }

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(floor_sqrt(0), 0);
        assert_eq!(floor_sqrt(1), 1);
        assert_eq!(floor_sqrt(3), 1);
        assert_eq!(floor_sqrt(4), 2);
        assert_eq!(floor_sqrt(99), 9);
        assert_eq!(floor_sqrt(100), 10);
        assert_eq!(floor_sqrt(120), 10);
    }
}
