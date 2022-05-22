use crate::{
    graph_edge_trait::{From, To, Value},
    negative_cycle::NegativeCycleError,
};

pub fn bellman_ford<E>(
    v_size: usize,
    directed_edges: &[E],
    src: usize,
) -> Result<Vec<Option<i64>>, NegativeCycleError>
where
    E: From<V = usize> + To<V = usize> + Value<T = i64>,
{
    let mut dist = vec![None; v_size];
    dist[src] = Some(0);
    for i in 0..v_size {
        for e in directed_edges {
            let u = *e.from();
            let v = *e.to();
            let w = *e.value();
            if dist[u].is_none()
                || dist[v].is_some() && dist[u].unwrap() + w >= dist[v].unwrap()
            {
                continue;
            }
            if i == v_size - 1 {
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
            bellman_ford(4, &edges, 1),
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
            bellman_ford(4, &edges, 0),
            Err(NegativeCycleError::new()),
        );
    }
}
