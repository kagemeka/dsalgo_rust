use crate::{
    bit_length::bit_length,
    tree_depths::tree_depths,
    tree_parents::tree_parents,
};

pub struct LCABinaryLifting {
    ancestors: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl LCABinaryLifting {
    pub fn new(tree_edges: &[(usize, usize)], root: usize) -> Self {
        let n = tree_edges.len() + 1;
        let depth = tree_depths(&tree_edges, root);
        let k = std::cmp::max(
            1,
            bit_length(*depth.iter().max().unwrap() as u64),
        ) as usize;
        let mut ancestors = vec![vec![n; n]; k];
        let mut parent = tree_parents(&tree_edges, root);
        parent[root] = Some(root);
        ancestors[0] = parent.iter().map(|&v| v.unwrap()).collect();
        for i in 0..k - 1 {
            for j in 0..n {
                ancestors[i + 1][j] = ancestors[i][ancestors[i][j]];
            }
        }
        Self { ancestors, depth }
    }

    pub fn get(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let d = self.depth[v] - self.depth[u];
        for i in 0..bit_length(d as u64) as usize {
            if d >> i & 1 == 1 {
                v = self.ancestors[i][v];
            }
        }
        if v == u {
            return u;
        }
        for a in self.ancestors.iter().rev() {
            let nu = a[u];
            let nv = a[v];
            if nu != nv {
                u = nu;
                v = nv;
            }
        }
        self.ancestors[0][u]
    }
}
