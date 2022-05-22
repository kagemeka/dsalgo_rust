/// O(E + V)
pub fn bfs(g: &[Vec<usize>], src: usize) -> Vec<usize> {
    let n: usize = g.len();
    let inf = std::usize::MAX;
    let mut dist = vec![inf; n];
    dist[src] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back(src);
    while let Some(u) = que.pop_front() {
        for &v in g[u].iter() {
            if dist[u] + 1 >= dist[v] {
                continue;
            }
            dist[v] = dist[u] + 1;
            que.push_back(v);
        }
    }
    dist
}

use crate::{
    dijkstra_dense::dijkstra_dense,
    negative_cycle::NegativeCycleError,
};
