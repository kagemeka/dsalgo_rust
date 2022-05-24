use crate::union_find::UnionFind;

pub fn connected_components_uf(
    v_size: usize,
    undirected_edges: &[(usize, usize)],
) -> Vec<usize> {
    let mut uf = UnionFind::new(v_size);
    for &(u, v) in undirected_edges {
        uf.unite(u, v);
    }
    uf.get_labels()
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
