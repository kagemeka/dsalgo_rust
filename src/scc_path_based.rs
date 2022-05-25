/// essentially same with Tarjan's Lowlink algorithm
pub fn scc_path_based(sparse_graph: &[Vec<usize>]) -> Vec<usize> {
    SCCPathBased::compute_labels(sparse_graph)
}

struct SCCPathBased<'a> {
    graph: &'a [Vec<usize>],
    stack: Vec<usize>,
    low_stack: Vec<usize>,
    orders: Vec<usize>,
    order: usize,
    labels: Vec<usize>,
    label: usize,
}

impl<'a> SCCPathBased<'a> {
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
            low_stack: vec![],
            orders: vec![n; n],
            order: 0,
            labels: vec![n; n],
            label: 0,
        }
    }

    fn size(&self) -> usize { self.graph.len() }

    fn labeling(scc: &mut Self, u: usize) {
        scc.orders[u] = scc.order;
        scc.order += 1;
        scc.stack.push(u);
        scc.low_stack.push(u);
        for &v in &scc.graph[u] {
            if scc.orders[v] == scc.size() {
                Self::labeling(scc, v);
            } else if scc.labels[v] == scc.size() {
                while scc.orders[*scc.low_stack.last().unwrap()] > scc.orders[v]
                {
                    scc.low_stack.pop();
                }
            }
        }
        if scc.low_stack.last().unwrap() != &u {
            return;
        }
        loop {
            let v = scc.stack.pop().unwrap();
            scc.labels[v] = scc.label;
            if v == u {
                break;
            }
        }
        scc.label += 1;
        scc.low_stack.pop();
    }

    fn topological_sort(labels: Vec<usize>) -> Vec<usize> {
        let k = *labels.iter().max().unwrap();
        labels.into_iter().map(|l| k - l).collect::<Vec<_>>()
    }
}

// #[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
