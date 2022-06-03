pub fn min_cost_elastic_match(cost_matrix: &[Vec<Option<i64>>]) -> Option<i64> {
    let n = cost_matrix.len();
    let m = cost_matrix[0].len();
    assert!((0..n).all(|i| cost_matrix[i].len() == m));
    let mut min_cost = vec![vec![None; m + 1]; n + 1];
    min_cost[0][0] = Some(0);
    for i in 0..n {
        for j in 0..m {
            if let Some(min) = [
                min_cost[i][j],
                min_cost[i + 1][j],
                min_cost[i][j + 1],
            ]
            .iter()
            .filter_map(|&c| c)
            .min()
            {
                if let Some(c) = cost_matrix[i][j] {
                    min_cost[i + 1][j + 1] = Some(min + c);
                }
            }
        }
    }
    min_cost[n][m]
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
