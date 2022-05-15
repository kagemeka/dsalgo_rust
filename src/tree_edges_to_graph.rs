pub fn tree_edges_with_data_to_graph<T>(
    tree_edges: &[(usize, usize, T)],
) -> Vec<Vec<(usize, T)>>
where
    T: Clone,
{
    let mut graph = vec![vec![]; tree_edges.len() + 1];
    for &(u, v, ref data) in tree_edges {
        graph[u].push((v, data.clone()));
        graph[v].push((u, data.clone()));
    }
    graph
}

pub fn tree_edges_to_graph(tree_edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    tree_edges_with_data_to_graph(
        tree_edges
            .iter()
            .map(|&(u, v)| (u, v, ()))
            .collect::<Vec<_>>()
            .as_slice(),
    )
    .iter()
    .map(|edges| edges.iter().map(|&(v, _)| v).collect())
    .collect()
}
