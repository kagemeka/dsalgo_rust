use crate::union_find::UnionFind;

// TODO: make edge type generic.
/// O(E\log{E})
pub fn mst_kruskcal(
    v_size: usize,
    undirected_edges: Vec<(usize, usize, i64)>,
) -> Vec<usize> {
    let mut uf = UnionFind::new(v_size);
    let mut edges: Vec<(usize, (usize, usize, i64))> =
        undirected_edges.into_iter().enumerate().collect();
    edges.sort_by_key(|&(_, (.., w))| w);
    let mut added = Vec::with_capacity(v_size - 1);
    for (i, (u, v, _)) in edges {
        if uf.are_same(u, v) {
            continue;
        }
        uf.unite(u, v);
        added.push(i);
    }
    debug_assert!(added.len() == v_size - 1);
    added
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
