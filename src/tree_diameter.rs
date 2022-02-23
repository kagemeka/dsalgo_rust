fn tree_dfs(
    g: &Vec<Vec<(usize, i64)>>,
    root: usize,
) -> (Vec<usize>, Vec<i64>) {
    let mut st = vec![root];
    let n = g.len();
    let mut parent = vec![n; n];
    let mut dist = vec![0; n];
    while let Some(u) = st.pop() {
        for &(v, w) in g[u].iter() {
            if v == parent[u] {
                continue;
            }
            parent[v] = u;
            dist[v] = dist[u] + w;
            st.push(v);
        }
    }
    (parent, dist)
}

pub fn tree_diameter(g: &Vec<(usize, usize, i64)>) -> (Vec<usize>, i64) {
    let n = g.len() + 1;
    let mut t: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(u, v, w) in g {
        t[u].push((v, w));
        t[v].push((u, w));
    }
    let (_, dist) = tree_dfs(&t, 0);
    let u = dist.iter().enumerate().max_by_key(|(_, &d)| d).unwrap().0;
    let (parent, dist) = tree_dfs(&t, u);
    let (mut v, &diameter) =
        dist.iter().enumerate().max_by_key(|&(v, d)| (d, v)).unwrap();
    let mut path = Vec::new();
    while v != n {
        path.push(v);
        v = parent[v];
    }
    (path, diameter)
}
