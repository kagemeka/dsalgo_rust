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

impl<V, E> Default for Node<V, E> {
    fn default() -> Self {
        Self {
            edges: Vec::new(),
            data: None,
        }
    }
}

#[derive(Debug)]
pub struct DirectedEdge<E = EdgeData, V = NodeData> {
    pub from: Rc<RefCell<Node<V, E>>>,
    pub to: Rc<RefCell<Node<V, E>>>,
    pub data: Option<E>,
}

impl<E, V> Edge<E, V> for DirectedEdge<E, V> {}

impl<E, V> From<(Rc<RefCell<Node<V, E>>>, Rc<RefCell<Node<V, E>>>)> for DirectedEdge<E, V> {
    fn from(nodes: (Rc<RefCell<Node<V, E>>>, Rc<RefCell<Node<V, E>>>)) -> Self {
        Self {
            from: nodes.0,
            to: nodes.1,
            data: None,
        }
    }
}

impl<E, V> DirectedEdge<E, V> {
    pub fn new(
        from: Rc<RefCell<Node<V, E>>>,
        to: Rc<RefCell<Node<V, E>>>,
        data: Option<E>,
    ) -> Self {
        Self { from, to, data }
    }
}

#[derive(Debug)]
pub struct UndirectedEdge<E = EdgeData, V = NodeData> {
    pub left: Rc<RefCell<Node<V, E>>>,
    pub right: Rc<RefCell<Node<V, E>>>,
    pub data: Option<E>,
}

impl<E, V> Edge<E, V> for UndirectedEdge<E, V> {}

impl<E, V> From<(Rc<RefCell<Node<V, E>>>, Rc<RefCell<Node<V, E>>>)> for UndirectedEdge<E, V> {
    fn from(nodes: (Rc<RefCell<Node<V, E>>>, Rc<RefCell<Node<V, E>>>)) -> Self {
        Self {
            left: nodes.0,
            right: nodes.1,
            data: None,
        }
    }
}

impl<E, V> UndirectedEdge<E, V> {
    pub fn new(
        left: Rc<RefCell<Node<V, E>>>,
        right: Rc<RefCell<Node<V, E>>>,
        data: Option<E>,
    ) -> Self {
        Self {
            left,
            right,
            data,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        use std::{cell::RefCell, rc::Rc};
        let node_left = Rc::new(RefCell::new(super::Node::default()));
        let node_right = Rc::new(RefCell::new(super::Node::default()));
        let edge = Rc::new(RefCell::new(super::DirectedEdge::<usize, usize>::new(
            node_left.clone(),
            node_right.clone(),
            None,
        )));
        let edge = Rc::new(RefCell::new(super::DirectedEdge::<usize, usize>::from((
            node_left.clone(),
            node_right.clone(),
        ))));
        node_left.borrow_mut().edges.push(edge.clone());
        println!("{:?}", edge);
    }
}
