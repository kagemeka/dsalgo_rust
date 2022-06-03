use crate::connected_components::connected_components_uf;

/// O(E\log{V})
pub fn mst_boruvka(
    v_size: usize,
    undirected_edges: &[(usize, usize, i64)],
) -> Vec<usize> {
    let m = undirected_edges.len();
    let mut is_added = vec![false; m];
    let mut added_eids = vec![];
    loop {
        let labels = connected_components_uf(
            v_size,
            &added_eids
                .iter()
                .map(|&i| {
                    let e: (usize, usize, i64) = undirected_edges[i];
                    (e.0, e.1)
                })
                .collect::<Vec<_>>(),
        );
        let k = *labels.iter().max().unwrap() + 1;
        if k == 1 {
            break;
        }
        let mut min_edge = vec![m; k];
        for (i, &(u, v, w)) in undirected_edges.iter().enumerate() {
            if is_added[i] {
                continue;
            }
            let u = labels[u];
            let v = labels[v];
            if u == v {
                continue;
            }
            if min_edge[u] == m || w < undirected_edges[min_edge[u]].2 {
                min_edge[u] = i;
            }
            if min_edge[v] == m || w < undirected_edges[min_edge[v]].2 {
                min_edge[v] = i;
            }
        }
        for i in min_edge {
            if i == m || is_added[i] {
                continue;
            }
            added_eids.push(i);
            is_added[i] = true;
        }
    }
    added_eids
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
