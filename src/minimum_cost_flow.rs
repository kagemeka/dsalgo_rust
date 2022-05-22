// TODO

// pub fn primal_dual(g: &[Vec<(usize, u64, i64)>]) ->
// Option<(i64, u64)> {     fn preprocess_graph(g:
// &[Vec<(usize, u64, i64)>]) -> Vec<Vec<(usize, u64, i64,
// usize)>> {         let n = g.len();
//         let mut t = vec![vec![]; n];
//         for u in 0..n {
//             for &(v, cap, cost) in g[u].iter() {
//                 t[u].push((v, cap, cost, t[v].len()));
//                 t[v].push((u, 0, -cost, t[u].len() - 1));
//             }
//         }
//         t
//     }
//     let g = preprocess_graph(g);

//     // bellman ford
//     // dijkstra

// }
