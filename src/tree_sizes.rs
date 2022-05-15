use crate::tree_dfs::tree_dfs;

pub fn tree_sizes(edges: &[(usize, usize)], root: usize) -> Vec<usize> {
    tree_dfs::<usize, _>(edges, root, vec![1; edges.len() + 1], |size, u, v| {
        size[u] += size[v];
    })
}
