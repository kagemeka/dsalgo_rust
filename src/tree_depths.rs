use crate::tree_bfs::tree_bfs;

pub fn tree_depths(edges: &[(usize, usize)], root: usize) -> Vec<usize> {
    tree_bfs::<usize, _>(
        edges,
        root,
        vec![0; edges.len() + 1],
        |depth, u, v| {
            depth[v] = depth[u] + 1;
        },
    )
}
