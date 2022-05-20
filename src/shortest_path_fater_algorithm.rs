/// SSSP with negative edges containing no cycles.
/// O(VE) but usually faster than bellman ford.
pub fn spfa(
    sparse_graph: &[Vec<(usize, i64)>],
    src: usize,
) -> Vec<Option<i64>> {
    let n = sparse_graph.len();
    let mut dist = vec![None; n];
    dist[src] = Some(0);
    let mut stack = vec![src];
    let mut on_stack = vec![false; n];
    on_stack[src] = true;
    while let Some(u) = stack.pop() {
        on_stack[u] = false;
        for &(v, w) in &sparse_graph[u] {
            let dv = Some(dist[u].unwrap() + w);
            if dist[v].is_some() && dv >= dist[v] {
                continue;
            }
            dist[v] = dv;
            if !on_stack[v] {
                stack.push(v);
                on_stack[v] = true;
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_negative_edge() {
        let graph = vec![
            vec![(1, 2), (2, 3)],
            vec![(2, -5), (3, 1)],
            vec![(3, 2)],
            vec![],
        ];
        assert_eq!(
            spfa(&graph, 1),
            vec![
                None,
                Some(0),
                Some(-5),
                Some(-3)
            ],
        );
    }
}
