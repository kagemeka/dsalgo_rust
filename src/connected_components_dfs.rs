pub fn connected_components_dfs(
    v_size: usize,
    undirected_edges: &[(usize, usize)],
) -> Vec<usize> {
    let mut g = vec![vec![]; v_size];
    for &(u, v) in undirected_edges.iter() {
        g[u].push(v);
        g[v].push(u);
    }
    let mut labels = vec![v_size; v_size];
    let mut label = 0;
    fn dfs(g: &[Vec<usize>], labels: &mut Vec<usize>, label: usize, u: usize) {
        labels[u] = label;
        for &v in &g[u] {
            if labels[v] == g.len() {
                dfs(g, labels, label, v);
            }
        }
    }
    for i in 0..v_size {
        if labels[i] != v_size {
            continue;
        }
        dfs(&g, &mut labels, label, i);
        label += 1;
    }
    labels
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
