use crate::graph_with_pointer::{EdgeData, NodeData};

pub struct DirectedEdge<T> {
    pub from: usize,
    pub to: usize,
    pub data: T,
}

pub struct DirectedGraph<T = NodeData, U = EdgeData> {
    pub nodes: Vec<T>,
    pub edges: Vec<Vec<DirectedEdge<U>>>,
}
