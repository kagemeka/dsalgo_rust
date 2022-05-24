pub fn is_adjacency_matrix<T>(graph: &[Vec<T>]) -> bool {
    let n = graph.len();
    graph.iter().all(|row| row.len() == n)
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
