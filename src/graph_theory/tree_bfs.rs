pub fn tree_bfs(
    g: &Vec<(usize, usize)>,
    root: usize,
) -> (Vec<usize>, Vec<usize>) {
    let n = g.len() + 1;
    let mut t = vec![vec![]; n];
    for &(u, v) in g.iter() {
        t[u].push(v);
        t[v].push(u);
    }
    let mut parent = vec![n; n];
    let mut depth = vec![0; n];
    let mut que = std::collections::VecDeque::new();
    que.push_back(root);
    while let Some(u) = que.pop_front() {
        for &v in t[u].iter() {
            if v == parent[u] {
                continue;
            }
            parent[v] = u;
            depth[v] = depth[u] + 1;
            que.push_back(v);
        }
    }
    (parent, depth)
}
