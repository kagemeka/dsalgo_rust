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

/// O((E + V)\log{E})
pub fn dijkstra_sparse(g: &Vec<Vec<(usize, i64)>>, src: usize) -> Vec<i64> {
    let n = g.len();
    let mut dist = vec![std::i64::MAX; n];
    dist[src] = 0;
    let mut hq = std::collections::BinaryHeap::<(i64, usize)>::new();
    hq.push((0, src));
    while let Some((mut du, u)) = hq.pop() {
        du = -du;
        if du > dist[u] {
            continue;
        }
        for (v, w) in g[u].iter() {
            let dv = du + w;
            if dv >= dist[*v] {
                continue;
            }
            dist[*v] = dv;
            hq.push((-dv, *v));
        }
    }
    dist
}

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

#[derive(Debug)]
pub struct NegativeCycleError {
    msg: &'static str,
}

impl NegativeCycleError {
    fn new() -> Self {
        Self {
            msg: "Negative Cycle Found.",
        }
    }
}

impl std::fmt::Display for NegativeCycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for NegativeCycleError {
    fn description(&self) -> &str { &self.msg }
}

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

/// O(V^3)
pub fn floyd_warshall(
    mut g: Vec<Vec<i64>>,
) -> Result<Vec<Vec<i64>>, NegativeCycleError> {
    let inf = std::i64::MAX;
    let n = g.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if g[i][k] == inf || g[k][j] == inf {
                    continue;
                }
                g[i][j] = std::cmp::min(g[i][j], g[i][k] + g[k][j]);
            }
        }
    }
    for i in 0..n {
        if g[i][i] < 0 {
            return Err(NegativeCycleError::new());
        }
    }
    Ok(g)
}
