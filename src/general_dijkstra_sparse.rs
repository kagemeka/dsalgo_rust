use crate::{
    dijkstra_sparse_queue::DijkstraSparseQueue,
    graph_edge_trait::{To, Weight},
};

pub fn general_dijkstra_sparse<E, T, F, Q>(
    sparse_graph: &[Vec<E>],
    callback: &F,
    mut data: T,
    src: usize,
) -> (Vec<Option<u64>>, T)
where
    E: To<V = usize> + Weight<u64>,
    F: Fn(&Vec<Option<u64>>, &mut T, usize, &E),
    Q: DijkstraSparseQueue,
{
    let n = sparse_graph.len();
    let mut hq = Q::default();
    let mut dist = vec![None; n];
    dist[src] = Some(0);
    hq.push((0, src));
    while let Some((du, u)) = hq.pop() {
        if du > dist[u].unwrap() {
            continue;
        }
        for e in sparse_graph[u].iter() {
            callback(&dist, &mut data, u, e);
            let v = *e.to();
            let dv = du + e.weight();
            if dist[v].is_some() && dv >= dist[v].unwrap() {
                continue;
            }
            dist[v] = Some(dv);
            hq.push((dv, v));
        }
    }
    (dist, data)
}

// TODO
// test on each specific problems
// like sssp, path-count, and predecessors, ....
#[cfg(test)]
mod tests {}
