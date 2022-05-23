use crate::{
    adjacency_list_graph::AdjacencyList,
    dijkstra_sparse_queue::DijkstraSparseQueue,
    graph_edge_trait::{From, To, Weight},
    sssp_dijkstra_sparse::dijkstra_sparse,
};

/// return edge ids.
pub fn dijkstra_arborescense<E, Q>(
    v_size: usize,
    directed_edges: &[E],
    root: usize,
) -> Vec<usize>
where
    E: From<V = usize> + To<V = usize> + Weight<u64> + Clone,
    Q: DijkstraSparseQueue,
{
    let g = AdjacencyList::from((v_size, directed_edges.to_vec()));
    let dist = dijkstra_sparse::<_, Q>(g.edges(), root);

    let mut visited = vec![false; v_size];
    visited[root] = true;
    let mut added = vec![];
    for (i, e) in directed_edges.iter().enumerate() {
        let u = *e.from();
        let v = *e.to();
        if dist[u].is_none()
            || visited[v]
            || dist[u].unwrap() + e.weight() > dist[v].unwrap()
        {
            continue;
        }
        visited[v] = true;
        added.push(i);
    }
    debug_assert_eq!(added.len(), v_size - 1);
    added
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
