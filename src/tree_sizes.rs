use crate::tree_dfs::tree_dfs;

pub fn tree_sizes(tree_edges: &[(usize, usize)], root: usize) -> Vec<usize> {
    tree_dfs::<usize, _>(
        tree_edges,
        root,
        vec![1; tree_edges.len() + 1],
        |size, u, v| {
            size[u] += size[v];
        },
    )
}
