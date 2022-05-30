pub fn str_to_byte_vec(s: &str) -> Vec<u8> { s.bytes().collect::<Vec<_>>() }
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let s = "abc";
        assert_eq!(
            str_to_byte_vec(s),
            vec![b'a', b'b', b'c'],
        );
    }
}
