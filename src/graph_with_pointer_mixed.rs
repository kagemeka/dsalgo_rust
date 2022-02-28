#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

pub(crate) struct EdgeData;
pub(crate) struct NodeData;

pub(crate) enum Edge<T = Option<EdgeData>, U = Option<NodeData>> {
    Directed {
        from: Rc<RefCell<Node<U, T>>>,
        to: Rc<RefCell<Node<U, T>>>,
        data: T,
    },
    Undirected {
        left: Rc<RefCell<Node<U, T>>>,
        right: Rc<RefCell<Node<U, T>>>,
        data: T,
    },
}

/// avoid cyclic reference
impl<T: std::fmt::Debug, U> std::fmt::Debug for Edge<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Edge::Directed {
                from: _,
                to: _,
                data,
            } => write!(f, "Edge::Directed {{ data: {:?} }}", data),
            Edge::Undirected {
                left: _,
                right: _,
                data,
            } => write!(f, "Edge::Undirected {{ data: {:?} }}", data),
        }
    }
}

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

#[derive(Debug)]
pub struct MixedGraph<T, U> {
    pub(crate) nodes: Vec<Rc<RefCell<Node<T, U>>>>,
}

impl<T, U> MixedGraph<T, U> {
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
        self.nodes.push(Rc::new(RefCell::new(Node::default())));
    }

    pub fn add_directed_edge(&mut self, from: usize, to: usize, data: U)
    where
        T: 'static,
        U: 'static,
    {
        assert!(from < self.size() && to < self.size());
        self.nodes[from]
            .borrow_mut()
            .edges
            .push(Rc::new(RefCell::new(Edge::Directed {
                from: self.nodes[from].clone(),
                to: self.nodes[to].clone(),
                data: data,
            })));
    }

    pub fn add_undirected_edge(&mut self, left: usize, right: usize, data: U)
    where
        T: 'static,
        U: 'static,
    {
        assert!(left < self.size() && right < self.size());
        let edge = Rc::new(RefCell::new(Edge::Undirected {
            left: self.nodes[left].clone(),
            right: self.nodes[right].clone(),
            data,
        }));
        self.nodes[left].borrow_mut().edges.push(edge.clone());
        self.nodes[right].borrow_mut().edges.push(edge.clone());
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        use std::{cell::RefCell, rc::Rc};

        let node_left = Rc::new(RefCell::new(super::Node::default()));
        let node_right = Rc::new(RefCell::new(super::Node::default()));
        let edge = Rc::new(RefCell::new(super::Edge::<(), usize>::Directed {
            from: node_left.clone(),
            to: node_right.clone(),
            data: (),
        }));
        println!("{:?}", edge);
        println!("{:?}", node_left);
        node_left.borrow_mut().edges.push(edge.clone());
        println!("{:?}", edge);
        println!("{:?}", node_left);
        let edge = Rc::new(RefCell::new(super::Edge::<(), usize>::Undirected {
            left: node_left.clone(),
            right: node_right.clone(),
            data: (),
        }));
        println!("{:?}", edge);
        println!("{:?}", node_left);

        let mut graph = super::MixedGraph::<(), usize>::new(2);
        println!("{:?}", graph);
        graph.add_directed_edge(0, 1, 3);
        println!("{:?}", graph);
        graph.add_undirected_edge(0, 1, 2);
        println!("{:?}", graph);
    }
}
