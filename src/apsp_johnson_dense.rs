use crate::{
    dijkstra_dense::dijkstra_dense,
    negative_cycle::NegativeCycleError,
    shortest_path_potential::shortest_path_potential,
};

/// O(V^3)
/// all pairs shortest path
pub fn johnson_dense(
    dense_graph: &[Vec<Option<i64>>],
) -> Result<Vec<Vec<Option<i64>>>, NegativeCycleError> {
    let n = dense_graph.len();
    let mut edges = vec![];
    for i in 0..n {
        for j in 0..n {
            if let Some(w) = dense_graph[i][j] {
                edges.push((i, j, w));
            }
        }
    }
    let p = shortest_path_potential(n, edges)?;
    let g = dense_graph
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(|(j, &d)| {
                    if d.is_some() {
                        Some((d.unwrap() - p[j] + p[i]) as u64)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut results = vec![];
    for i in 0..n {
        let dist = dijkstra_dense(&g, i)
            .into_iter()
            .enumerate()
            .map(|(j, d)| d.map(|x| x as i64 + p[j] - p[i]))
            .collect();
        results.push(dist);
    }
    Ok(results)
}

// TODO
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
            johnson_dense(&g),
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
            johnson_dense(&g),
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
            johnson_dense(&g),
            Err(NegativeCycleError::new()),
        )
    }
}
