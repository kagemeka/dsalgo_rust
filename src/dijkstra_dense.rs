// TODO: refactor
/// O(V^2)
pub fn dijkstra_dense(g: &Vec<Vec<i64>>, src: usize) -> Vec<i64> {
    let n = g.len();
    let inf = std::i64::MAX;
    let mut dist = vec![inf; n];
    dist[src] = 0;
    let mut visited = vec![false; n];
    loop {
        let mut u = -1;
        let mut du = inf;
        for i in 0..n {
            if !visited[i] && dist[i] < du {
                u = i as i32;
                du = dist[i];
            }
        }
        if u == -1 {
            break;
        }
        let u = u as usize;
        visited[u] = true;
        for v in 0..n {
            if g[u][v] == inf {
                continue;
            }
            let dv = du + g[u][v];
            if dv >= dist[v] {
                continue;
            }
            dist[v] = dv;
        }
    }
    dist
}
