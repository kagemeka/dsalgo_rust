use crate::{
    algebra::{abstract_::structure::structs::*, bitwise::bit_length},
    graph_theory::{
        euler_tour::euler_tour_node,
        tree::segment_tree::SegmentTree,
        tree_bfs::tree_bfs,
    },
    sparse_table::*,
    union_find::UnionFind,
};

pub fn tarjan_offline(
    g: &Vec<(usize, usize)>,
    uv: &Vec<(usize, usize)>,
    root: usize,
) -> Vec<usize> {
    fn dfs(
        g: &Vec<Vec<usize>>,
        q: &Vec<Vec<(usize, usize)>>,
        visited: &mut Vec<bool>,
        uf: &mut UnionFind,
        ancestor: &mut Vec<usize>,
        lca: &mut Vec<usize>,
        u: usize,
    ) {
        visited[u] = true;
        ancestor[u] = u;
        for &v in g[u].iter() {
            if visited[v] {
                continue;
            }
            dfs(g, q, visited, uf, ancestor, lca, v);
            uf.unite(u, v);
            ancestor[uf.find(u)] = u;
        }
        for &(v, i) in q[u].iter() {
            if visited[v] {
                lca[i] = ancestor[uf.find(v)];
            }
        }
    }
    let n = g.len() + 1;
    let mut t = vec![vec![]; n];
    for &(u, v) in g.iter() {
        t[u].push(v);
        t[v].push(u);
    }
    let mut q = vec![vec![]; n];
    for (i, &(u, v)) in uv.iter().enumerate() {
        q[u].push((v, i));
        q[v].push((u, i));
    }
    let mut visited = vec![false; n];
    let mut uf = UnionFind::new(n);
    let mut ancestor = vec![n; n];
    let mut lca = vec![n; uv.len()];
    dfs(&t, &q, &mut visited, &mut uf, &mut ancestor, &mut lca, root);
    lca
}

pub struct BinaryLifting {
    ancestor: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl BinaryLifting {
    pub fn new(g: &Vec<(usize, usize)>, root: usize) -> Self {
        let n = g.len() + 1;
        let (parent, depth) = tree_bfs(g, root);
        let k = std::cmp::max(1, bit_length(*depth.iter().max().unwrap()));
        let mut ancestor = vec![vec![n; n]; k];
        ancestor[0] = parent;
        ancestor[0][root] = root;
        for i in 0..k - 1 {
            for j in 0..n {
                ancestor[i + 1][j] = ancestor[i][ancestor[i][j]];
            }
        }
        Self {
            ancestor: ancestor,
            depth: depth,
        }
    }

    pub fn get(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let d = self.depth[v] - self.depth[u];
        for i in 0..bit_length(d) {
            if d >> i & 1 == 1 {
                v = self.ancestor[i][v];
            }
        }
        if v == u {
            return u;
        }
        for a in self.ancestor.iter().rev() {
            let nu = a[u];
            let nv = a[v];
            if nu != nv {
                u = nu;
                v = nv;
            }
        }
        self.ancestor[0][u]
    }
}

pub fn with_hl_decomposition() {}

pub mod eulertour_rmq {

    use super::{
        euler_tour_node,
        DisjointSparseTable,
        Monoid,
        SegmentTree,
        Semigroup,
        SparseTable,
    };

    pub struct WithSparseTable<'a, S> {
        first_idx: Vec<usize>,
        sp: DisjointSparseTable<'a, S>,
    }
    impl<'a> WithSparseTable<'a, (usize, usize)> {
        pub fn new(g: &Vec<(usize, usize)>, root: usize) -> Self {
            let (tour, first_idx, _, _, depth) = euler_tour_node(g, root);
            let sg = Semigroup::<'a, (usize, usize)> {
                op: &|x, y| std::cmp::min(*x, *y),
                commutative: true,
                idempotent: true,
            };
            let mut a = Vec::with_capacity(tour.len());
            for &i in tour.iter() {
                a.push((depth[i as usize], i as usize));
            }
            let sp = DisjointSparseTable::new(sg, &a);
            Self {
                first_idx: first_idx,
                sp: sp,
            }
        }

        pub fn get(&self, u: usize, v: usize) -> usize {
            let mut l = self.first_idx[u];
            let mut r = self.first_idx[v];
            if l > r {
                std::mem::swap(&mut l, &mut r);
            }
            self.sp.get(l, r + 1).1
        }
    }

    pub struct WithSegmentTree<'a, S: Copy> {
        first_idx: Vec<usize>,
        seg: SegmentTree<'a, S>,
    }
    impl<'a> WithSegmentTree<'a, (usize, usize)> {
        pub fn new(g: &Vec<(usize, usize)>, root: usize) -> Self {
            let (tour, first_idx, _, _, depth) = euler_tour_node(g, root);
            let m = Monoid::<'a, (usize, usize)> {
                op: &|x, y| std::cmp::min(*x, *y),
                e: &|| (std::usize::MAX, 0),
                commutative: true,
                idempotent: false,
            };
            let mut a = Vec::with_capacity(tour.len());
            for &i in tour.iter() {
                a.push((depth[i as usize], i as usize));
            }
            let seg = SegmentTree::from_vec(m, &a);
            Self {
                first_idx: first_idx,
                seg: seg,
            }
        }

        pub fn get(&self, u: usize, v: usize) -> usize {
            let mut l = self.first_idx[u];
            let mut r = self.first_idx[v];
            if l > r {
                std::mem::swap(&mut l, &mut r);
            }
            self.seg.get(l, r + 1).1
        }
    }

    pub struct WithSqrtDecomposition {}
}

pub struct WithHLD {}
