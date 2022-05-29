use crate::longest_common_subsequence_dp_table::lcs_dp_table;

pub fn lcs_length(a: &[u8], b: &[u8]) -> usize {
    lcs_dp_table(a, b)[a.len()][b.len()]
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
