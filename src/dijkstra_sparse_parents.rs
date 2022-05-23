use crate::{
    dijkstra_sparse_queue::DijkstraSparseQueue,
    general_dijkstra_sparse::general_dijkstra_sparse,
    graph_edge_trait::{To, Weight},
};

pub fn dijkstra_sparse_parents<E, Q>(
    sparse_graph: &[Vec<E>],
    src: usize,
) -> Vec<Option<usize>>
where
    E: To<V = usize> + Weight<u64>,
    Q: DijkstraSparseQueue,
{
    general_dijkstra_sparse::<_, _, _, Q>(
        sparse_graph,
        &|dist, parent: &mut Vec<Option<usize>>, u, e: &E| {
            let v = *e.to();
            let dv = dist[u].unwrap() + e.weight();
            if dist[v].is_none() || dv < dist[v].unwrap() {
                parent[v] = Some(u);
            }
        },
        vec![None; sparse_graph.len()],
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
