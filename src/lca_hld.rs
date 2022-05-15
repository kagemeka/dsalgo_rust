use crate::{
    heavy_light_decomposition::heavy_light_decompose,
    tree_depths::tree_depths,
    tree_parents::tree_parents,
};

pub struct LCAHLD {
    parent: Vec<Option<usize>>,
    depth: Vec<usize>,
    roots: Vec<usize>,
}

impl LCAHLD {
    pub fn new(tree_edges: &[(usize, usize)], root: usize) -> Self {
        Self {
            parent: tree_parents(tree_edges, root),
            depth: tree_depths(tree_edges, root),
            roots: heavy_light_decompose(tree_edges, root),
        }
    }

    pub fn get(&self, mut u: usize, mut v: usize) -> usize {
        while self.roots[u] != self.roots[v] {
            if self.depth[self.roots[u]] > self.depth[self.roots[v]] {
                std::mem::swap(&mut u, &mut v);
            }
            v = self.parent[self.roots[v]].unwrap();
        }
        if self.depth[u] <= self.depth[v] { u } else { v }
    }
}
