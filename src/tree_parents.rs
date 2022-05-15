use crate::tree_bfs::tree_bfs;

pub fn tree_parents(edges: &[(usize, usize)], root: usize) -> Vec<Option<usize>> {
    tree_bfs::<Option<usize>, _>(
        edges,
        root,
        vec![None; edges.len() + 1],
        |parent, u, v| {
            parent[v] = Some(u);
        },
    )
}
