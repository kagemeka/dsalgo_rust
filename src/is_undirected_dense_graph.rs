use crate::is_adjacency_matrix::is_adjacency_matrix;

pub fn is_undirected_dense_graph<T: Ord>(graph: &[Vec<T>]) -> bool {
    if !is_adjacency_matrix(graph) {
        return false;
    }
    let n = graph.len();
    (0..n - 1).all(|i| (i + 1..n).all(|j| graph[i][j] == graph[j][i]))
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
