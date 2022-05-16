use crate::negative_cycle::NegativeCycleError;

pub fn floyd_warshall(
    mut weight_matrix: Vec<Vec<i64>>,
) -> Result<Vec<Vec<i64>>, NegativeCycleError> {
    let n = weight_matrix.len();
    assert!((0..n).all(|i| weight_matrix[i].len() == n));
    (0..n).for_each(|i| {
        weight_matrix[i][i] = std::cmp::min(weight_matrix[i][i], 0)
    });
    (0..n).for_each(|k| {
        (0..n).for_each(|i| {
            (0..n).for_each(|j| {
                weight_matrix[i][j] = std::cmp::min(
                    weight_matrix[i][j],
                    weight_matrix[i][k] + weight_matrix[k][j],
                );
            });
        });
    });
    if (0..n).any(|i| weight_matrix[i][i] < 0) {
        Err(NegativeCycleError::new())
    } else {
        Ok(weight_matrix)
    }
}
