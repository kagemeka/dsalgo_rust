pub fn lsb(n: u64) -> usize {
    assert!(n > 0);
    n.trailing_zeros() as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn bitwise() {
        assert_eq!(super::lsb(1), 0,);
    }
}
