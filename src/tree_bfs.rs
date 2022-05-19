use crate::tree_edges_to_graph::tree_edges_to_graph;

pub fn tree_bfs<T, F>(
    tree_edges: &[(usize, usize)],
    root: usize,
    default_data: Vec<T>,
    mut assign: F,
) -> Vec<T>
where
    F: FnMut(&mut Vec<T>, usize, usize),
{
    let graph = tree_edges_to_graph(tree_edges);
    let n = graph.len();
    assert!(default_data.len() == n);
    let mut que = std::collections::VecDeque::new();
    let mut parent = vec![None; n];
    let mut data = default_data;
    que.push_back(root);
    while let Some(u) = que.pop_front() {
        for &v in graph[u].iter() {
            if Some(v) == parent[u] {
                continue;
            }
            parent[v] = Some(u);
            assign(&mut data, u, v);
            que.push_back(v);
        }
    }
    data
}
