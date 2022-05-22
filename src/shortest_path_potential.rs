use crate::{
    adjacency_list_graph::AdjacencyList,
    graph_edge_trait::{From, To, Weight},
    negative_cycle::NegativeCycleError,
    spfa::spfa,
};

pub trait ShortestPathPotentialEdge:
    From<V = usize>
    + To<V = usize>
    + Weight<i64>
    + std::convert::From<(usize, usize, i64)>
{
}

impl<T> ShortestPathPotentialEdge for T where
    T: From<V = usize>
        + To<V = usize>
        + Weight<i64>
        + std::convert::From<(usize, usize, i64)>
{
}

/// used to map edge weights from i64 to u64 space.
/// mainly spawned as preprocessing
/// before calling Dijkstra's algorithm multiple times
/// on a graph which might be containing negative weighted edges.
pub fn shortest_path_potential<E>(
    v_size: usize,
    mut directed_edges: Vec<E>,
) -> Result<Vec<i64>, NegativeCycleError>
where
    E: ShortestPathPotentialEdge,
{
    directed_edges.extend((0..v_size).map(|i| (v_size, i, 0).into()));
    let g = AdjacencyList::<E>::from((v_size + 1, directed_edges));
    match spfa(g.edges(), v_size) {
        Ok(mut potential) => {
            potential.pop();
            Ok(potential.into_iter().map(|x| x.unwrap()).collect())
        },
        Err(e) => Err(e),
    }
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
