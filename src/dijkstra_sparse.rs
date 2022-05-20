// TODO: use generic type for priority queues instead of binary heap.
/// O((E + V)\log{E})
pub fn dijkstra_sparse(g: &Vec<Vec<(usize, i64)>>, src: usize) -> Vec<i64> {
    let n = g.len();
    let mut dist = vec![std::i64::MAX; n];
    dist[src] = 0;
    let mut hq = std::collections::BinaryHeap::<(i64, usize)>::new();
    hq.push((0, src));
    while let Some((mut du, u)) = hq.pop() {
        du = -du;
        if du > dist[u] {
            continue;
        }
        for (v, w) in g[u].iter() {
            let dv = du + w;
            if dv >= dist[*v] {
                continue;
            }
            dist[*v] = dv;
            hq.push((-dv, *v));
        }
    }
    dist
}
