pub fn str_to_char_vec(s: &str) -> Vec<char> { s.chars().collect::<Vec<_>>() }

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let s = "abc";
        assert_eq!(
            str_to_char_vec(s),
            vec!['a', 'b', 'c']
        );
    }
}
