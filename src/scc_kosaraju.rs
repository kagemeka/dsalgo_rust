pub fn scc_kosaraju(sparse_graph: Vec<Vec<usize>>) -> Vec<usize> {
    SCCKosaraju::compute_labels(sparse_graph)
}

struct SCCKosaraju {
    graph: Vec<Vec<usize>>,
    visited: Vec<bool>,
    topological_reverse_order: Vec<usize>,
    labels: Vec<usize>,
}

impl SCCKosaraju {
    fn compute_labels(directed_sparse_graph: Vec<Vec<usize>>) -> Vec<usize> {
        let mut scc = SCCKosaraju::new(directed_sparse_graph);

        for u in 0..scc.size() {
            if !scc.visited[u] {
                scc.compute_topological_rev_ord(u);
            }
        }
        scc.transpose_graph();
        let mut label = 0;
        for u in scc.topological_reverse_order.clone().into_iter().rev() {
            if scc.labels[u] != scc.size() {
                continue;
            }
            scc.labeling(label, u);
            label += 1;
        }
        scc.labels
    }

    fn new(directed_sparse_graph: Vec<Vec<usize>>) -> Self {
        let n = directed_sparse_graph.len();
        Self {
            graph: directed_sparse_graph,
            visited: vec![false; n],
            topological_reverse_order: vec![],
            labels: vec![n; n],
        }
    }

    fn size(&self) -> usize { self.graph.len() }

    fn transpose_graph(&mut self) {
        let n = self.size();
        let mut g = vec![vec![]; n];
        for i in 0..n {
            for &j in &self.graph[i] {
                g[j].push(i);
            }
        }
        self.graph = g;
    }

    fn compute_topological_rev_ord(&mut self, u: usize) {
        self.visited[u] = true;
        for v in self.graph[u].clone().into_iter() {
            if !self.visited[v] {
                Self::compute_topological_rev_ord(self, v);
            }
        }
        self.topological_reverse_order.push(u);
    }

    // components are fixed in topological order for initial(user-given) edges.
    fn labeling(&mut self, label: usize, u: usize) {
        self.labels[u] = label;
        for v in self.graph[u].clone().into_iter() {
            if self.labels[v] == self.size() {
                self.labeling(label, v);
            }
        }
    }
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
