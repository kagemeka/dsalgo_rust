/// O(V^2)
pub fn dijkstra_dense(
    dense_graph: &[Vec<Option<u64>>],
    src: usize,
) -> Vec<Option<u64>> {
    let g = dense_graph;
    let n = g.len();
    let mut dist = vec![None; n];
    dist[src] = Some(0);
    let mut visited = vec![false; n];
    loop {
        let mut u = None;
        let mut du = None;
        for i in 0..n {
            if visited[i] || dist[i].is_none() {
                continue;
            }
            if du.is_none() || dist[i] < du {
                u = Some(i);
                du = dist[i];
            }
        }
        if u.is_none() {
            break;
        }
        let u = u.unwrap();
        let du = du.unwrap();
        visited[u] = true;
        for v in 0..n {
            if g[u][v].is_none() {
                continue;
            }
            let dv = Some(du + g[u][v].unwrap());
            if dist[v].is_none() || dv < dist[v] {
                dist[v] = dv;
            }
        }
    }
    dist
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
