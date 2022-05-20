// TODO: use generic type for priority queues instead of binary heap.
pub fn dijkstra_sparse(
    sparse_graph: &[Vec<(usize, i64)>],
    src: usize,
) -> Vec<Option<i64>> {
    use std::cmp::Reverse;
    let n = sparse_graph.len();
    let mut dist = vec![None; n];
    let mut hq = std::collections::BinaryHeap::<Reverse<(i64, _)>>::new();
    dist[src] = Some(0);
    hq.push(Reverse((0, src)));
    while let Some(Reverse((du, u))) = hq.pop() {
        if du > dist[u].unwrap() {
            continue;
        }
        for &(v, weight) in sparse_graph[u].iter() {
            let dv = du + weight;
            if dist[v].is_some() && dv >= dist[v].unwrap() {
                continue;
            }
            dist[v] = Some(dv);
            hq.push(Reverse((dv, v)));
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let g = vec![
            vec![(1, 1), (2, 4)],
            vec![(2, 2), (3, 5)],
            vec![(3, 1)],
            vec![],
        ];
        assert_eq!(
            super::dijkstra_sparse(&g, 0),
            vec![
                Some(0),
                Some(1),
                Some(3),
                Some(4)
            ]
        );

        let g = vec![
            vec![(1, 1), (2, 4)],
            vec![(2, 2)],
            vec![(0, 1)],
            vec![(1, 1), (2, 5)],
        ];
        assert_eq!(
            super::dijkstra_sparse(&g, 1),
            vec![Some(3), Some(0), Some(2), None]
        );
    }
}
