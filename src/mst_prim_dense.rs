use crate::is_undirected_dense_graph::is_undirected_dense_graph;

/// return edges list (u, v), u < v
/// the algorithm start from vertex 0.
/// O(V^2)
pub fn mst_prim_dense(
    undirected_dense_graph: &[Vec<Option<i64>>],
) -> Vec<(usize, usize)> {
    let g = undirected_dense_graph;
    let n = g.len();
    assert!(is_undirected_dense_graph(g));
    let mut prev: Vec<Option<usize>> = vec![None; n];
    let mut visited = vec![false; n];
    let mut added = Vec::with_capacity(n - 1);
    for _ in 0..n {
        let (pre, u, _) = (0..n)
            .filter_map(|i| {
                if visited[i] || prev[i].is_none() {
                    return None;
                }
                let p = prev[i].unwrap();
                Some((
                    p,
                    i,
                    g[prev[i].unwrap()][i].unwrap(),
                ))
            })
            .min_by_key(|&(.., w)| w)
            .unwrap_or_default();
        visited[u] = true;
        if pre != u {
            if pre > u {
                let mut pre = pre;
                let mut u = u;
                std::mem::swap(&mut pre, &mut u);
            }
            added.push((pre, u));
        }
        for v in 0..n {
            if visited[v]
                || g[u][v].is_none()
                || prev[v].is_some() && g[u][v] >= g[prev[v].unwrap()][v]
            {
                continue;
            }
            prev[v] = Some(u);
        }
    }
    debug_assert_eq!(added.len(), n - 1);
    added
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
