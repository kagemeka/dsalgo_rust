use crate::{tree_edges_to_graph::tree_edges_to_graph, tree_sizes::tree_sizes};

pub fn heavy_light_decompose(
    tree_edges: &[(usize, usize)],
    root: usize,
) -> Vec<usize> {
    let graph = tree_edges_to_graph(tree_edges);
    let n = graph.len();
    let mut roots = (0..n).collect::<Vec<_>>();
    let sizes = tree_sizes(tree_edges, root);
    let mut stack = vec![(root, root)];
    while let Some((u, parent)) = stack.pop() {
        let mut heavy_node = None;
        let mut max_size = 0;
        graph[u].iter().filter(|&&v| v != parent).for_each(|&v| {
            if sizes[v] > max_size {
                max_size = sizes[v];
                heavy_node = Some(v);
            }
        });
        graph[u].iter().filter(|&&v| v != parent).for_each(|&v| {
            if Some(v) == heavy_node {
                roots[v] = roots[u];
            }
            stack.push((v, u));
        });
    }
    roots
}
