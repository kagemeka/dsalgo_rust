/// restore one of the transtion histories.
pub(crate) fn restore_lcs_indices(
    lcs_dp_table: &[Vec<usize>],
) -> Vec<(usize, usize)> {
    let mut indices = vec![];
    let length = lcs_dp_table;
    let mut i = length.len() - 1;
    let mut j = length[0].len() - 1;
    while i > 0 && j > 0 {
        let l = length[i][j];
        if length[i][j - 1] == l {
            j -= 1;
            continue;
        }
        if length[i - 1][j] == l {
            i -= 1;
            continue;
        }
        i -= 1;
        j -= 1;
        indices.push((i, j));
    }
    indices.reverse();
    indices
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::longest_common_subsequence_dp_table::lcs_dp_table;
        let s = "axyb".chars().collect::<Vec<_>>();
        let t = "abyxb".chars().collect::<Vec<_>>();
        let lcs = lcs_dp_table(&s, &t);
        let indices = restore_lcs_indices(&lcs);
        assert_eq!(
            indices,
            vec![(0, 0), (2, 2), (3, 4)]
        );
        // ayb
    }
}
