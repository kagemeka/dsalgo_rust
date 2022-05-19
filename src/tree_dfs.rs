use crate::tree_edges_to_graph::tree_edges_to_graph;

/// we don't check if the given edges as a whole making a tree or not.
/// because the algorithm might be expensive even if O(N).
/// if not graph, the processes is undefined.
/// similary, we assure that the default data length
/// is equial to the number of vertices.
pub fn tree_dfs<T, F>(
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
    let mut parent = vec![None; n];
    let mut data = default_data;
    let mut stack: Vec<isize> = Vec::new();
    stack.push(root as isize);
    while let Some(u) = stack.pop() {
        if u < 0 {
            let u = !u as usize;
            if let Some(p) = parent[u] {
                assign(&mut data, p, u);
            }
            continue;
        }
        stack.push(!u);
        let u = u as usize;
        for &v in graph[u].iter() {
            if Some(v) == parent[u] {
                continue;
            }
            parent[v] = Some(u);
            stack.push(v as isize);
        }
    }
    data
}
