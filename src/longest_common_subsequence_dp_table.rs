pub fn lcs_dp_table<T: PartialEq>(a: &[T], b: &[T]) -> Vec<Vec<usize>> {
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

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
