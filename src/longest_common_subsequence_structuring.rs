use crate::longest_common_subsequence_dp_table::lcs_dp_table;

pub fn struct_lcs<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let n = a.len();
    let m = b.len();
    let length = lcs_dp_table(a, b);
    let mut lcs = Vec::with_capacity(length[n][m]);

    let mut i = n;
    let mut j = m;
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
        lcs.push(a[i].clone());
    }
    lcs.reverse();
    lcs
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
