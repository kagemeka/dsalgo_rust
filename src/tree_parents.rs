use crate::tree_bfs::tree_bfs;

pub fn tree_parents(
    tree_edges: &[(usize, usize)],
    root: usize,
) -> Vec<Option<usize>> {
    tree_bfs::<Option<usize>, _>(
        tree_edges,
        root,
        vec![None; tree_edges.len() + 1],
        |parent, u, v| {
            parent[v] = Some(u);
        },
    )
}
