//! Minimum Spanning Tree
use crate::{graph_theory::connected_components, union_find::UnionFind};


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
