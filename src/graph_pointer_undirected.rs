#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub(crate) struct Node<T, U> {
    pub(crate) edges: Vec<Rc<RefCell<Edge<U, T>>>>,
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
    pub(crate) lhs: Rc<RefCell<Node<U, T>>>,
    pub(crate) rhs: Rc<RefCell<Node<U, T>>>,
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
pub struct UndirectedGraph<T, U> {
    pub(crate) nodes: Vec<Rc<RefCell<Node<T, U>>>>,
}

impl<T, U> UndirectedGraph<T, U> {
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

    pub fn add_edge(&mut self, lhs: usize, rhs: usize, data: U) {
        assert!(lhs < self.size() && rhs < self.size());
        let edge = Rc::new(RefCell::new(Edge {
            lhs: self.nodes[lhs].clone(),
            rhs: self.nodes[rhs].clone(),
            data,
        }));
        self.nodes[lhs].borrow_mut().edges.push(edge.clone());
        self.nodes[rhs].borrow_mut().edges.push(edge.clone());
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let mut graph = super::UndirectedGraph::<(), usize>::new(2);
        println!("{:?}", graph);
        graph.add_edge(0, 1, 1);
        println!("{:?}", graph)
    }
}
