pub fn lcs_dp<T: PartialEq>(a: &[T], b: &[T]) -> Vec<Vec<usize>> {
    let n = a.len();
    let m = b.len();
    let mut length = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            if a[i] == b[j] {
                length[i + 1][j + 1] = length[i][j] + 1;
                continue;
            }
            length[i + 1][j + 1] = std::cmp::max(
                length[i][j + 1],
                length[i + 1][j],
            );
        }
    }
    length
}

pub fn struct_lcs<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let n = a.len();
    let m = b.len();
    let length = lcs_dp(a, b);
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
