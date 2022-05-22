use crate::{
    dijkstra_sparse_queue::DijkstraSparseQueue,
    general_dijkstra_sparse::general_dijkstra_sparse,
    graph_edge_trait::{To, Weight},
};

pub fn dijkstra_sparse<E, Q>(
    sparse_graph: &[Vec<E>],
    src: usize,
) -> Vec<Option<u64>>
where
    E: To<V = usize> + Weight<u64>,
    Q: DijkstraSparseQueue,
{
    general_dijkstra_sparse::<_, _, _, Q>(
        sparse_graph,
        &|_, _, _, _| {},
        (),
        src,
    )
    .0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dijkstra_queue_binary_heap_std::DijkstraQueueBinaryHeapStd;
    #[test]
    fn test() {
        let g = vec![
            vec![(0, 1, 1), (0, 2, 4)],
            vec![(1, 2, 2), (1, 3, 5)],
            vec![(2, 3, 1)],
            vec![],
        ];
        assert_eq!(
            dijkstra_sparse::<_, DijkstraQueueBinaryHeapStd>(&g, 0),
            vec![
                Some(0),
                Some(1),
                Some(3),
                Some(4)
            ]
        );

        let g = vec![
            vec![(0, 1, 1), (0, 2, 4)],
            vec![(1, 2, 2)],
            vec![(2, 0, 1)],
            vec![(3, 1, 1), (3, 2, 5)],
        ];
        assert_eq!(
            dijkstra_sparse::<_, DijkstraQueueBinaryHeapStd>(&g, 1),
            vec![Some(3), Some(0), Some(2), None]
        );
    }
}
