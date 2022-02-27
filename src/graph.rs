use std::{cell::RefCell, rc::Rc};

pub struct Graph {}

pub struct EdgeData;
pub struct NodeData;

pub trait Edge<E = EdgeData, V = NodeData> {}

pub struct Node<V = NodeData, E = EdgeData> {
    pub edges: Vec<Rc<RefCell<dyn Edge<E, V>>>>,
    pub data: Option<V>,
}

impl<V: std::fmt::Debug, E> std::fmt::Debug for Node<V, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ data: {:?} }}", self.data)
    }
}

#[derive(Debug)]
pub struct DirectedEdge<E = EdgeData, V = NodeData> {
    pub from: Rc<RefCell<Node<V, E>>>,
    pub to: Rc<RefCell<Node<V, E>>>,
    pub data: Option<E>,
}

impl<E, V> Edge<E, V> for DirectedEdge<E, V> {}
#[derive(Debug)]

pub struct UndirectedEdge<E = EdgeData, V = NodeData> {
    pub left: Rc<RefCell<Node<V, E>>>,
    pub right: Rc<RefCell<Node<V, E>>>,
    pub data: Option<E>,
}

impl<E, V> Edge<E, V> for UndirectedEdge<E, V> {}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        use std::{cell::RefCell, rc::Rc};
        let node_left = Rc::new(RefCell::new(super::Node::<usize, usize> {
            edges: Vec::new(),
            data: None,
        }));
        let node_right = Rc::new(RefCell::new(super::Node::<usize, usize> {
            edges: Vec::new(),
            data: None,
        }));

        let edge = Rc::new(RefCell::new(super::DirectedEdge::<usize, usize> {
            from: node_left.clone(),
            to: node_right.clone(),
            data: None,
        }));
        node_left.clone().borrow_mut().edges.push(edge.clone());
        println!("{:?}", edge);
    }
}
