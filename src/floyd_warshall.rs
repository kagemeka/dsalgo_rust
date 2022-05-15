use crate::negative_cycle::NegativeCycleError;

pub fn floyd_warshall(
    mut weight_matrix: Vec<Vec<i64>>,
) -> Result<Vec<Vec<i64>>, NegativeCycleError> {
    let n = weight_matrix.len();
    assert!((0..n).all(|i| weight_matrix[i].len() == n));
    for i in 0..n {
        weight_matrix[i][i] = std::cmp::min(weight_matrix[i][i], 0);
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                weight_matrix[i][j] = std::cmp::min(
                    weight_matrix[i][j],
                    weight_matrix[i][k] + weight_matrix[k][j],
                );
            }
        }
    }
    for i in 0..n {
        if weight_matrix[i][i] < 0 {
            return Err(NegativeCycleError::new());
        }
    }
    Ok(weight_matrix)
}
