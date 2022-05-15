use crate::{euler_tour_edges::euler_tour_edges, tree_parents::tree_parents};

pub fn euler_tour_nodes(
    tree_edges: &[(usize, usize)],
    root: usize,
) -> Vec<usize> {
    let parent = tree_parents(tree_edges, root);
    euler_tour_edges(tree_edges, root)
        .iter()
        .rev()
        .skip(1)
        .rev()
        .map(
            |&u| {
                if u < 0 { parent[!u as usize].unwrap() } else { u as usize }
            },
        )
        .collect()
}
