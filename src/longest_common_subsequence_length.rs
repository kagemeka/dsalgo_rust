use crate::longest_common_subsequence_dp_table::lcs_dp_table;

pub fn lcs_length<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    lcs_dp_table(a, b)[a.len()][b.len()]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let s = "axyb".chars().collect::<Vec<_>>();
        let t = "abyxb".chars().collect::<Vec<_>>();
        assert_eq!(lcs_length(&s, &t), 3);
    }
}
