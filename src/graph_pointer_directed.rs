#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub(crate) struct Node<T, U> {
    pub(crate) edges: Vec<Box<Edge<U, T>>>,
    pub(crate) data: T,
}

impl<T: Default, U> Default for Node<T, U> {
    fn default() -> Self {
        Self {
            edges: Vec::new(),
            data: T::default(),
        }
    }
}

pub(crate) struct Edge<T, U> {
    pub(crate) from: Rc<RefCell<Node<U, T>>>,
    pub(crate) to: Rc<RefCell<Node<U, T>>>,
    pub(crate) data: T,
}

/// avoid cycle reference
impl<T: std::fmt::Debug, U> std::fmt::Debug for Edge<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Edge {{ data: {:?} }}",
            self.data
        )
    }
}

#[derive(Debug)]
pub struct DirectedGraph<T, U> {
    pub(crate) nodes: Vec<Rc<RefCell<Node<T, U>>>>,
}

impl<T, U> DirectedGraph<T, U> {
    pub fn size(&self) -> usize { self.nodes.len() }

    pub fn new(size: usize) -> Self
    where
        T: Default,
    {
        Self {
            nodes: (0..size)
                .map(|_| Rc::new(RefCell::new(Node::default())))
                .collect(),
        }
    }

    pub fn add_node(&mut self)
    where
        T: Default,
    {
        self.nodes.push(Rc::new(RefCell::new(
            Node::default(),
        )));
    }

    pub fn add_edge(&mut self, from: usize, to: usize, data: U) {
        assert!(from < self.size() && to < self.size());
        self.nodes[from].borrow_mut().edges.push(Box::new(Edge {
            from: self.nodes[from].clone(),
            to: self.nodes[to].clone(),
            data,
        }));
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let mut graph = super::DirectedGraph::<(), usize>::new(2);
        println!("{:?}", graph);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 0, 2);
        println!("{:?}", graph)
    }
}
