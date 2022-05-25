use crate::undirected_lowlink::undirected_lowlink;

/// return edge ids.
pub fn find_bridges_lowlink(
    v_size: usize,
    undirected_edges: &[(usize, usize)],
) -> Vec<usize> {
    let lowlink = undirected_lowlink(v_size, undirected_edges);
    let order = lowlink.orders;
    let low = lowlink.low_orders;
    (0..undirected_edges.len())
        .filter(|&i| {
            let (mut u, mut v) = undirected_edges[i];
            if order[u] > order[v] {
                std::mem::swap(&mut u, &mut v);
            }
            low[v] == order[v]
        })
        .collect()
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
