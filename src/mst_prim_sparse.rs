// TODO
// use generic minimum queue
// use generic edge type

pub fn mst_prim_sparse(
    v_size: usize,
    undirected_edges: &[(usize, usize, i64)],
) -> Vec<usize> {
    use std::cmp::Reverse;
    let mut g = vec![vec![]; v_size];
    for (i, &(u, v, w)) in undirected_edges.iter().enumerate() {
        g[u].push((v, w, i));
        g[v].push((u, w, i));
    }

    let mut hq = std::collections::BinaryHeap::new();
    hq.push((Reverse(0), 0)); // weight, to
    let mut eid_to: Vec<Option<usize>> = vec![None; v_size];
    let mut visited = vec![false; v_size];
    while let Some((_, u)) = hq.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        for &(v, w, i) in &g[u] {
            if visited[v]
                || eid_to[v].is_some()
                    && w >= undirected_edges[eid_to[v].unwrap()].2
            {
                continue;
            }
            hq.push((Reverse(w), v));
            eid_to[v] = Some(i);
        }
    }
    eid_to.into_iter().filter_map(|e| e).collect()
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
