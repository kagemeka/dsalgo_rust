pub fn least_significant_bit(n: u64) -> Option<usize> { if n == 0 { None } else { Some(n.trailing_zeros() as usize) } }

#[cfg(test)]
mod tests {
    #[test]
    fn bitwise() {
        assert_eq!(
            super::least_significant_bit(0),
            None
        );
        assert_eq!(
            super::least_significant_bit(1),
            Some(0)
        );
    }
}
