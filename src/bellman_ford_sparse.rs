use crate::negative_cycle::NegativeCycleError;

pub fn bellman_ford_sparse(
    nodes_size: usize,
    directed_edges: &[(usize, usize, i64)],
    src: usize,
) -> Result<Vec<Option<i64>>, NegativeCycleError> {
    let mut dist = vec![None; nodes_size];
    dist[src] = Some(0);
    for i in 0..nodes_size {
        for &(u, v, w) in directed_edges {
            if dist[u].is_none()
                || dist[v].is_some() && dist[u].unwrap() + w >= dist[v].unwrap()
            {
                continue;
            }
            if i == nodes_size - 1 {
                return Err(NegativeCycleError::new());
            }
            dist[v] = Some(dist[u].unwrap() + w);
        }
    }
    Ok(dist)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_negative_edge() {
        let edges = vec![
            (0, 1, 2),
            (0, 2, 3),
            (1, 2, -5),
            (1, 3, 1),
            (2, 3, 2),
        ];
        assert_eq!(
            bellman_ford_sparse(4, &edges, 1),
            Ok(vec![
                None,
                Some(0),
                Some(-5),
                Some(-3)
            ]),
        );
    }

    #[test]
    fn test_negative_cycle() {
        let edges = vec![
            (0, 1, 2),
            (0, 2, 3),
            (1, 2, -5),
            (1, 3, 1),
            (2, 3, 2),
            (3, 1, 0),
        ];
        assert_eq!(
            bellman_ford_sparse(4, &edges, 0),
            Err(NegativeCycleError::new()),
        );
    }
}
