
pub struct BinaryLifting {
    ancestors: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

// impl BinaryLifting {
//     pub fn new(g: &Vec<(usize, usize)>, root: usize) -> Self {
//         let n = g.len() + 1;
//         let (parent, depth) = tree_bfs(g, root);
//         let k = std::cmp::max(1, bit_length(*depth.iter().max().unwrap()));
//         let mut ancestor = vec![vec![n; n]; k];
//         ancestor[0] = parent;
//         ancestor[0][root] = root;
//         for i in 0..k - 1 {
//             for j in 0..n {
//                 ancestor[i + 1][j] = ancestor[i][ancestor[i][j]];
//             }
//         }
//         Self {
//             ancestor: ancestor,
//             depth: depth,
//         }
//     }

//     pub fn get(&self, mut u: usize, mut v: usize) -> usize {
//         if self.depth[u] > self.depth[v] {
//             std::mem::swap(&mut u, &mut v);
//         }
//         let d = self.depth[v] - self.depth[u];
//         for i in 0..bit_length(d) {
//             if d >> i & 1 == 1 {
//                 v = self.ancestor[i][v];
//             }
//         }
//         if v == u {
//             return u;
//         }
//         for a in self.ancestor.iter().rev() {
//             let nu = a[u];
//             let nv = a[v];
//             if nu != nv {
//                 u = nu;
//                 v = nv;
//             }
//         }
//         self.ancestor[0][u]
//     }
// }