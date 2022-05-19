use crate::{tree_edges_to_graph::tree_edges_to_graph, union_find::UnionFind};

pub fn offline_lca_tarjan(
    tree_edges: &[(usize, usize)],
    queries: &[(usize, usize)],
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
            dfs(
                g, q, visited, uf, ancestor, lca, v,
            );
            uf.unite(u, v);
            ancestor[uf.find_root(u)] = u;
        }
        q[u].iter().filter(|&&(v, _)| visited[v]).for_each(|&(v, i)| {
            lca[i] = ancestor[uf.find_root(v)];
        });
    }
    let n = tree_edges.len() + 1;
    let graph = tree_edges_to_graph(tree_edges);
    let mut q = vec![vec![]; n];
    for (i, &(u, v)) in queries.iter().enumerate() {
        q[u].push((v, i));
        q[v].push((u, i));
    }
    let mut visited = vec![false; n];
    let mut uf = UnionFind::new(n);
    let mut ancestor = vec![n; n];
    let mut lca = vec![n; queries.len()];
    dfs(
        &graph,
        &q,
        &mut visited,
        &mut uf,
        &mut ancestor,
        &mut lca,
        root,
    );
    lca
}
