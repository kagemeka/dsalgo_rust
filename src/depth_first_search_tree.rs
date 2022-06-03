use crate::{
    depth_first_search_arborescence::dfs_arborescense,
    graph_edge_trait::{From, Reversed, To, ToDirected},
    undirected_edges_to_directed::edges_to_directed,
};

/// return edge ids
pub fn dfs_tree<E1, E2>(
    v_size: usize,
    undirected_edges: &[E1],
    root: usize,
) -> Vec<usize>
where
    E1: From<V = usize> + To<V = usize> + Clone + ToDirected<E = E2>,
    E2: Reversed + From<V = usize> + To<V = usize>,
{
    let m = undirected_edges.len();
    let edges = edges_to_directed(undirected_edges.to_vec());
    dfs_arborescense(v_size, &edges, root)
        .into_iter()
        .map(|i| i % m)
        .collect()
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
