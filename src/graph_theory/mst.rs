//! Minimum Spanning Tree
use crate::{graph_theory::connected_components, union_find::UnionFind};

/// O(E\log{E})
pub fn kruskal(
    n: usize,
    mut g: Vec<(usize, usize, i64)>,
) -> Vec<(usize, usize, i64)> {
    g.sort_by_key(|x| x.2);
    let mut uf = UnionFind::new(n);
    let mut mst = Vec::with_capacity(n - 1);
    for (u, v, w) in g.into_iter() {
        if uf.find(u) == uf.find(v) {
            continue;
        }
        uf.unite(u, v);
        mst.push((u, v, w));
    }
    mst
}

/// O((E + V)\log{E})
pub fn prim_sparse(
    n: usize,
    g: &Vec<(usize, usize, i64)>,
) -> Vec<(usize, usize, i64)> {
    let mut t: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for (u, v, w) in g.iter() {
        t[*u].push((*v, *w));
        t[*v].push((*u, *w));
    }
    let mut mst: Vec<(usize, usize, i64)> = Vec::with_capacity(n - 1);
    let mut hq = std::collections::BinaryHeap::new();
    hq.push((0, 0, 0));
    let inf = std::i64::MAX;
    let mut weight = vec![inf; n];
    let mut visited = vec![false; n];
    while let Some((wu, pre, u)) = hq.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        if pre != u {
            mst.push((pre, u, -wu));
        }
        for (v, wv) in t[u].iter() {
            if visited[*v] || *wv >= weight[*v] {
                continue;
            }
            weight[*v] = *wv;
            hq.push((-wv, u, *v));
        }
    }
    mst
}

/// O(V^2)
pub fn prim_dense(g: &Vec<Vec<i64>>) -> Vec<(usize, usize, i64)> {
    let n = g.len();
    for u in 1..n {
        for v in 0..u {
            assert_eq!(g[u][v], g[v][u]);
        }
    }
    let inf = std::i64::MAX;
    let mut mst: Vec<(usize, usize, i64)> = Vec::with_capacity(n - 1);
    let mut min_edge = vec![(n, inf); n];
    min_edge[0] = (0, 0);
    let mut visited = vec![false; n];
    for _ in 0..n {
        let mut pre = n;
        let mut u = n;
        let mut wu = inf;
        for i in 0..n {
            if visited[i] || min_edge[i].1 >= wu {
                continue;
            }
            u = i;
            pre = min_edge[i].0;
            wu = min_edge[i].1
        }
        assert!(wu < inf);
        visited[u] = true;
        if pre != u {
            mst.push((pre, u, wu));
        }
        for v in 0..n {
            if visited[v] || g[u][v] >= min_edge[v].1 {
                continue;
            }
            min_edge[v] = (u, g[u][v]);
        }
    }
    assert_eq!(mst.len(), n - 1);
    mst
}

/// O(E\log{V})
pub fn boruvka(
    n: usize,
    g: &Vec<(usize, usize, i64)>,
) -> Vec<(usize, usize, i64)> {
    let m = g.len();
    let mut is_added = vec![false; g.len()];
    let mut mst: Vec<(usize, usize, i64)> = Vec::with_capacity(n - 1);
    loop {
        let label = connected_components::with_union_find(
            n,
            &mst.iter().map(|x| (x.0, x.1)).collect(),
        );
        let k = *label.iter().max().unwrap() + 1;
        if k == 1 {
            break;
        }
        let mut min_edge = vec![m; k];
        for (i, (u, v, w)) in g.iter().enumerate() {
            let u = label[*u];
            let v = label[*v];
            if u == v {
                continue;
            }
            if min_edge[u] == m || *w < g[min_edge[u]].2 {
                min_edge[u] = i;
            }
            if min_edge[v] == m || *w < g[min_edge[v]].2 {
                min_edge[v] = i;
            }
        }
        for i in min_edge {
            if is_added[i] {
                continue;
            }
            mst.push(g[i]);
            is_added[i] = true;
        }
    }
    mst
}

pub fn reverse_delete() {}

pub fn randomized_linear() {}
