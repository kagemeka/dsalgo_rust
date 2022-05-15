use crate::tree_edges_to_graph::tree_edges_to_graph;

pub fn euler_tour_edges(
    tree_edges: &[(usize, usize)],
    root: usize,
) -> Vec<isize> {
    let graph = tree_edges_to_graph(tree_edges);
    let n = graph.len();
    let mut parent = vec![None; n];
    let mut tour = Vec::with_capacity(n << 1);
    let mut stack = vec![root as isize];
    for _ in 0..n << 1 {
        let u = stack.pop().unwrap();
        tour.push(u);
        if u < 0 {
            continue;
        }
        stack.push(!u);
        let u = u as usize;
        graph[u].iter().rev().for_each(|&v| {
            if Some(v) != parent[u] {
                parent[v] = Some(u);
                stack.push(v as isize);
            }
        });
    }
    tour
}
