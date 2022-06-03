pub fn connected_components_bfs(
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
    for i in 0..v_size {
        if labels[i] != v_size {
            continue;
        }
        labels[i] = label;
        let mut que = std::collections::VecDeque::new();
        que.push_back(i);
        while let Some(u) = que.pop_front() {
            for &v in &g[u] {
                if labels[v] != v_size {
                    continue;
                }
                labels[v] = label;
                que.push_back(v);
            }
        }
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
