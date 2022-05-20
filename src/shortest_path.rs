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

/// O(VE)
pub fn bellman_ford_sparse(
    g: &Vec<Vec<(usize, i64)>>,
    src: usize,
) -> Result<Vec<i64>, NegativeCycleError> {
    let n = g.len();
    let inf = std::i64::MAX;
    let mut dist = vec![inf; n];
    dist[src] = 0;
    for _ in 0..n - 1 {
        for u in 0..n {
            for (v, w) in g[u].iter() {
                if dist[u] == inf || dist[u] + w >= dist[*v] {
                    continue;
                }
                dist[*v] = dist[u] + w;
            }
        }
    }
    for u in 0..n {
        for (v, w) in g[u].iter() {
            if dist[u] == inf || dist[u] + w >= dist[*v] {
                continue;
            }
            return Err(NegativeCycleError::new());
        }
    }
    Ok(dist)
}

/// O(V^3)
pub fn bellman_ford_dense(
    g: &Vec<Vec<i64>>,
    src: usize,
) -> Result<Vec<i64>, NegativeCycleError> {
    let n = g.len();
    let inf = std::i64::MAX;
    let mut dist = vec![inf; n];
    dist[src] = 0;
    for _ in 0..n - 1 {
        for u in 0..n {
            for v in 0..n {
                if dist[u] == inf
                    || g[u][v] == inf
                    || dist[u] + g[u][v] >= dist[v]
                {
                    continue;
                }
                dist[v] = dist[u] + g[u][v];
            }
        }
    }
    for u in 0..n {
        for v in 0..n {
            if dist[u] == inf || g[u][v] == inf || dist[u] + g[u][v] >= dist[v]
            {
                continue;
            }
            return Err(NegativeCycleError::new());
        }
    }
    Ok(dist)
}

/// O(V^3)
pub fn johnson_dense(
    g: &Vec<Vec<i64>>,
) -> Result<Vec<Vec<i64>>, NegativeCycleError> {
    let n = g.len();
    let inf = std::i64::MAX;
    let mut t = g.to_vec();
    t.push(vec![0; n + 1]);
    for i in 0..n {
        t[i].push(inf);
    }
    let h = bellman_ford_dense(&t, n)?;
    t = g.to_vec();
    for u in 0..n {
        for v in 0..n {
            if t[u][v] != inf {
                t[u][v] += h[u] - h[v];
            }
        }
    }
    let mut dist = Vec::with_capacity(n);
    for i in 0..n {
        let mut d = dijkstra_dense(&t, i);
        for j in 0..n {
            if d[j] != inf {
                d[j] -= h[i] - h[j];
            }
        }
        dist.push(d);
    }
    Ok(dist)
}
