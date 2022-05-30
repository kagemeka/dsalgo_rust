pub fn bytes_to_char_vec<I: Iterator<Item = u8>>(
    bytes: I,
    offset: u8,
) -> Vec<char> {
    bytes.map(|b| (b + offset) as char).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            bytes_to_char_vec(vec![0, 1, 2].into_iter(), b'a'),
            vec!['a', 'b', 'c'],
        );
    }
}
