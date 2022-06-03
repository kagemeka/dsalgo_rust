use crate::{
    dijkstra_sparse_queue::DijkstraSparseQueue,
    general_dijkstra_sparse::general_dijkstra_sparse,
    graph_edge_trait::{To, Weight},
};

/// if 0-cost edges are included, result does not always become a DAG.
pub fn dijkstra_sparse_predecessors<E, Q>(
    sparse_graph: &[Vec<E>],
    src: usize,
) -> Vec<Vec<usize>>
where
    E: To<V = usize> + Weight<u64>,
    Q: DijkstraSparseQueue,
{
    general_dijkstra_sparse::<_, _, _, Q>(
        sparse_graph,
        &|dist, pre: &mut Vec<Vec<usize>>, u, e: &E| {
            let v = *e.to();
            let dv = dist[u].unwrap() + e.weight();
            if let Some(prev_dv) = dist[v] {
                if dv < prev_dv {
                    pre[v] = vec![u];
                } else if dv == prev_dv {
                    pre[v].push(u);
                }
            } else {
                pre[v] = vec![u];
            }
        },
        vec![vec![]; sparse_graph.len()],
        src,
    )
    .1
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
