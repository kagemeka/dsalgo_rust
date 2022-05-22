use crate::{
    graph_edge_trait::{To, Weight},
    negative_cycle::NegativeCycleError,
};

/// O(VE) but usually faster than bellman ford.
pub fn spfa<E>(
    sparse_graph: &[Vec<E>],
    src: usize,
) -> Result<Vec<Option<i64>>, NegativeCycleError>
where
    E: To<V = usize> + Weight<i64>,
{
    let n = sparse_graph.len();
    let mut dist = vec![None; n];
    dist[src] = Some(0);
    let mut que = vec![src];
    let mut in_que = vec![false; n];
    in_que[src] = true;
    for _ in 0..n {
        let mut next_que = vec![];
        for u in que.into_iter() {
            in_que[u] = false;
            for e in &sparse_graph[u] {
                let v = *e.to();
                let dv = Some(dist[u].unwrap() + e.weight());
                if dist[v].is_some() && dv >= dist[v] {
                    continue;
                }
                dist[v] = dv;
                if !in_que[v] {
                    next_que.push(v);
                    in_que[v] = true;
                }
            }
        }
        que = next_que;
        if que.is_empty() {
            return Ok(dist);
        }
    }
    Err(NegativeCycleError::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_negative_edge() {
        let graph = vec![
            vec![(0, 1, 2), (0, 2, 3)],
            vec![(1, 2, -5), (1, 3, 1)],
            vec![(2, 3, 2)],
            vec![],
        ];
        assert_eq!(
            spfa(&graph, 1),
            Ok(vec![
                None,
                Some(0),
                Some(-5),
                Some(-3)
            ]),
        );
    }
}
