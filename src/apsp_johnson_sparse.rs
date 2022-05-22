use crate::{
    adjacency_list_graph::AdjacencyList,
    dijkstra_sparse_queue::DijkstraSparseQueue,
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
    use super::*;
    use crate::dijkstra_queue_binary_heap_std::DijkstraQueueBinaryHeapStd as Q;
    #[test]
    fn test_positive() {
        let edges = vec![
            (0, 1, 1),
            (0, 2, 5),
            (1, 2, 2),
            (1, 3, 4),
            (2, 3, 1),
            (3, 2, 7),
        ];
        assert_eq!(
            johnson_sparse::<_, Q>(4, edges),
            Ok(vec![
                vec![
                    Some(0),
                    Some(1),
                    Some(3),
                    Some(4)
                ],
                vec![None, Some(0), Some(2), Some(3)],
                vec![None, None, Some(0), Some(1)],
                vec![None, None, Some(7), Some(0)],
            ]),
        )
    }

    #[test]
    fn test_negative() {
        let edges = vec![
            (0, 1, 1),
            (0, 2, -5),
            (1, 2, 2),
            (1, 3, 4),
            (2, 3, 1),
            (3, 2, 7),
        ];
        assert_eq!(
            johnson_sparse::<_, Q>(4, edges),
            Ok(vec![
                vec![
                    Some(0),
                    Some(1),
                    Some(-5),
                    Some(-4)
                ],
                vec![None, Some(0), Some(2), Some(3)],
                vec![None, None, Some(0), Some(1)],
                vec![None, None, Some(7), Some(0)],
            ]),
        )
    }
    #[test]
    fn test_negative_cycle() {
        let edges = vec![
            (0, 1, 1),
            (0, 2, 5),
            (1, 2, 2),
            (1, 3, 4),
            (2, 3, 1),
            (3, 2, -7),
        ];
        assert_eq!(
            johnson_sparse::<_, Q>(4, edges),
            Err(NegativeCycleError::new()),
        )
    }
}
