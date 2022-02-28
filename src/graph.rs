use crate::graph_with_pointer::{EdgeData, NodeData};

pub struct DirectedEdge<T> {
    pub from: usize,
    pub to: usize,
    pub data: T,
}

pub struct DirectedGraph<T, U> {
    pub nodes: Vec<Option<T>>,
    pub edges: Vec<Vec<DirectedEdge<U>>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let a = Option::<usize>::default();
        println!("{}", a.is_none());
    }
}
