pub fn chars_to_byte_vec<I: Iterator<Item = char>>(
    chars: I,
    offset: u8,
) -> Vec<u8> {
    chars.map(|c| c as u8 - offset).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let s = "abc";

        assert_eq!(
            chars_to_byte_vec(s.chars(), b'a'),
            vec![0, 1, 2]
        );
    }
}
