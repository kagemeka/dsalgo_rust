// TODO: refactor
use crate::{
    bellman_ford_dense::bellman_ford_dense,
    dijkstra_dense::dijkstra_dense,
    negative_cycle::NegativeCycleError,
};

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
