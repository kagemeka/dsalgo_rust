use crate::{
    adjacency_list_graph::AdjacencyList,
    dijkstra_sparse_queue::DijkstraSparseQueue,
    graph_edge_trait::Weight,
    negative_cycle::NegativeCycleError,
    shortest_path_potential::{
        shortest_path_potential,
        ShortestPathPotentialEdge,
    },
    sssp_dijkstra_sparse::dijkstra_sparse,
};

pub fn johnson_sparse<E, Q>(
    v_size: usize,
    directed_edges: Vec<E>,
) -> Result<Vec<Vec<Option<i64>>>, NegativeCycleError>
where
    E: ShortestPathPotentialEdge + Clone,
    Q: DijkstraSparseQueue,
{
    let p = shortest_path_potential::<E>(v_size, directed_edges.clone())?;
    type E2 = (usize, usize, u64);
    let edges = directed_edges
        .into_iter()
        .map(|e| {
            let u = *e.from();
            let v = *e.to();
            let w: u64 = (e.weight() - p[v] + p[u]) as u64;
            (u, v, w)
        })
        .collect::<Vec<E2>>();
    let g = AdjacencyList::<E2>::from((v_size, edges));
    let mut results = vec![];
    for i in 0..v_size {
        let dist = dijkstra_sparse::<E2, Q>(g.edges(), i)
            .into_iter()
            .enumerate()
            .map(|(j, d)| d.map(|x| x as i64 + p[j] - p[i]))
            .collect();
        results.push(dist);
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
