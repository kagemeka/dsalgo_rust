use crate::negative_cycle::NegativeCycleError;

pub fn floyd_warshall(
    weight_matrix: Vec<Vec<Option<i64>>>,
) -> Result<Vec<Vec<Option<i64>>>, NegativeCycleError> {
    let mut g = weight_matrix;
    let n = g.len();
    assert!((0..n).all(|i| g[i].len() == n));
    for i in 0..n {
        if g[i][i].is_none() || g[i][i] > Some(0) {
            g[i][i] = Some(0)
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if g[i][k].is_none() || g[k][j].is_none() {
                    continue;
                }
                let d = Some(g[i][k].unwrap() + g[k][j].unwrap());
                if g[i][j].is_none() || d < g[i][j] {
                    g[i][j] = d;
                }
            }
        }
    }
    if (0..n).any(|i| g[i][i] < Some(0)) {
        Err(NegativeCycleError::new())
    } else {
        Ok(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive() {
        let g = vec![
            vec![None, Some(1), Some(5), None],
            vec![None, None, Some(2), Some(4)],
            vec![None, None, None, Some(1)],
            vec![None, None, Some(7), None],
        ];
        assert_eq!(
            floyd_warshall(g),
            Ok(vec![
                vec![
                    Some(0),
                    Some(1),
                    Some(3),
                    Some(4)
                ],
                vec![None, Some(0), Some(2), Some(3)],
                vec![None, None, Some(0), Some(1)],
                vec![None, None, Some(7), Some(0)],
            ]),
        )
    }

    #[test]
    fn test_negative() {
        let g = vec![
            vec![None, Some(1), Some(-5), None],
            vec![None, None, Some(2), Some(4)],
            vec![None, None, None, Some(1)],
            vec![None, None, Some(7), None],
        ];
        assert_eq!(
            floyd_warshall(g),
            Ok(vec![
                vec![
                    Some(0),
                    Some(1),
                    Some(-5),
                    Some(-4)
                ],
                vec![None, Some(0), Some(2), Some(3)],
                vec![None, None, Some(0), Some(1)],
                vec![None, None, Some(7), Some(0)],
            ]),
        )
    }
    #[test]
    fn test_negative_cycle() {
        let g = vec![
            vec![None, Some(1), Some(5), None],
            vec![None, None, Some(2), Some(4)],
            vec![None, None, None, Some(1)],
            vec![None, None, Some(-7), None],
        ];
        assert_eq!(
            floyd_warshall(g),
            Err(NegativeCycleError::new()),
        )
    }
}
