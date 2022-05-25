/// essentially same with Path-Based algorithm
pub fn scc_tarjan_lowlink(sparse_graph: &[Vec<usize>]) -> Vec<usize> {
    SCCTarjanLowLink::compute_labels(sparse_graph)
}

// just a data wrapper to avoid having many parameters in a dfs function.
struct SCCTarjanLowLink<'a> {
    graph: &'a [Vec<usize>],
    stack: Vec<usize>,
    orders: Vec<usize>,
    order: usize,
    low_order: Vec<usize>,
    labels: Vec<usize>,
    label: usize,
}

impl<'a> SCCTarjanLowLink<'a> {
    pub fn compute_labels(
        directed_sparse_graph: &'a [Vec<usize>],
    ) -> Vec<usize> {
        let mut scc = Self::new(directed_sparse_graph);
        for i in 0..scc.size() {
            if scc.labels[i] == scc.size() {
                Self::labeling(&mut scc, i);
            }
        }
        Self::topological_sort(scc.labels)
    }

    fn new(graph: &'a [Vec<usize>]) -> Self {
        let n = graph.len();
        Self {
            graph,
            stack: vec![],
            orders: vec![n; n],
            order: 0,
            low_order: vec![n; n],
            labels: vec![n; n],
            label: 0,
        }
    }

    fn size(&self) -> usize { self.graph.len() }

    // labels are fixed in topologically reverse order of components.
    fn labeling(scc: &mut Self, u: usize) {
        scc.orders[u] = scc.order;
        scc.order += 1;
        scc.stack.push(u);
        for &v in &scc.graph[u] {
            if scc.orders[v] == scc.size() {
                Self::labeling(scc, v);
                scc.low_order[u] = std::cmp::min(
                    scc.low_order[u],
                    scc.low_order[v],
                );
            } else if scc.labels[v] == scc.size() {
                // v is not in a scc yet.
                scc.low_order[u] =
                    std::cmp::min(scc.low_order[u], scc.orders[v]);
                // `s.t. if low[v] < low[u] then low[u] = low[v]`?
                // but low[v] is still under computing.
                // here,
                // it's just enough to know whether
                // low[u] can be smaller than or equal to ord[v].
            }
        }
        if scc.low_order[u] < scc.orders[u] {
            return;
        }
        // a scc is fixed.
        loop {
            let v = scc.stack.pop().unwrap();
            scc.labels[v] = scc.label;
            if v == u {
                break;
            }
        }
        scc.label += 1;
    }

    fn topological_sort(labels: Vec<usize>) -> Vec<usize> {
        // after labeling, labels are still reverse order
        // in a point of topologicality.
        let k = *labels.iter().max().unwrap();
        labels.into_iter().map(|l| k - l).collect::<Vec<_>>()
    }
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
