// TODO: refactor
use crate::negative_cycle::NegativeCycleError;

/// O(V^3)
pub(crate) fn bellman_ford_dense(
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
