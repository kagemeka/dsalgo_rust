pub fn char_slice_to_string(s: &[char]) -> String { s.iter().collect() }

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let s = vec!['a', 'b', 'c'];
        assert_eq!(char_slice_to_string(&s), "abc");
    }
}
