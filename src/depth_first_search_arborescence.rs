use crate::graph_edge_trait::{From, To};

/// return edge ids.
pub fn dfs_arborescense<E>(
    v_size: usize,
    directed_edges: &[E],
    root: usize,
) -> Vec<usize>
where
    E: From<V = usize> + To<V = usize>,
{
    let mut g = vec![vec![]; v_size];
    for (id, e) in directed_edges.iter().enumerate() {
        let u = *e.from();
        let v = *e.to();
        g[u].push((v, id));
    }
    let mut visited = vec![false; v_size];
    let mut added_edges = vec![];

    fn dfs(
        g: &Vec<Vec<(usize, usize)>>,
        visited: &mut Vec<bool>,
        added_edges: &mut Vec<usize>,
        u: usize,
    ) {
        visited[u] = true;
        for &(v, i) in &g[u] {
            if visited[u] {
                continue;
            }
            added_edges.push(i);
            dfs(g, visited, added_edges, v);
        }
    }
    dfs(
        &g,
        &mut visited,
        &mut added_edges,
        root,
    );

    added_edges
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
