use crate::tree_edges_to_graph::tree_edges_to_graph;

// #[derive(Debug)]
// pub struct TreeBFS {
//     pub parent: Vec<Option<usize>>,
//     pub depth: Vec<usize>,
// }

// impl TreeBFS {
//     pub fn new(tree_edges: &[(usize, usize)], root: usize) -> Self {
//         let n = tree_edges.len() + 1;
//         let graph = tree_edges_to_graph(tree_edges);
//         let mut parent = vec![None; n];
//         let mut depth = vec![0; n];
//         let mut que = std::collections::VecDeque::new();
//         que.push_back(root);
//         while let Some(u) = que.pop_front() {
//             for &v in graph[u].iter() {
//                 if u != root && v == parent[u].unwrap() {
//                     continue;
//                 }
//                 parent[v] = Some(u);
//                 depth[v] = depth[u] + 1;
//                 que.push_back(v);
//             }
//         }
//         Self { parent, depth }
//     }
// }

pub fn tree_bfs<T, F>(edges: &[(usize, usize)], root: usize, default_data: Vec<T>, mut assign: F) -> Vec<T>
where
    F: FnMut(&mut Vec<T>, usize, usize),
{
    let graph = tree_edges_to_graph(edges);
    let n = graph.len();
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
