use std::{cell::RefCell, rc::Rc};

pub struct Graph {}

pub struct EdgeData;
pub struct NodeData;

pub trait Edge<T = Option<EdgeData>, U = Option<NodeData>> {}

pub struct Node<T, U> {
    pub edges: Vec<Rc<RefCell<dyn Edge<U, T>>>>,
    pub data: T,
}

impl<T: std::fmt::Debug, U> std::fmt::Debug for Node<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ data: {:?} }}", self.data)
    }
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
pub struct DirectedEdge<T, U> {
    pub from: Rc<RefCell<Node<U, T>>>,
    pub to: Rc<RefCell<Node<U, T>>>,
    pub data: T,
}

impl<T, U> Edge<T, U> for DirectedEdge<T, U> {}

impl<T: Default, U> From<(Rc<RefCell<Node<U, T>>>, Rc<RefCell<Node<U, T>>>)>
    for DirectedEdge<T, U>
{
    fn from(nodes: (Rc<RefCell<Node<U, T>>>, Rc<RefCell<Node<U, T>>>)) -> Self {
        Self {
            from: nodes.0,
            to: nodes.1,
            data: T::default(),
        }
    }
}

impl<T, U> DirectedEdge<T, U> {
    pub fn new(from: Rc<RefCell<Node<U, T>>>, to: Rc<RefCell<Node<U, T>>>, data: T) -> Self {
        Self { from, to, data }
    }
}

#[derive(Debug)]
pub struct UndirectedEdge<T, U> {
    pub left: Rc<RefCell<Node<U, T>>>,
    pub right: Rc<RefCell<Node<U, T>>>,
    pub data: T,
}

impl<T, U> Edge<T, U> for UndirectedEdge<T, U> {}

impl<T: Default, U> From<(Rc<RefCell<Node<U, T>>>, Rc<RefCell<Node<U, T>>>)>
    for UndirectedEdge<T, U>
{
    fn from(nodes: (Rc<RefCell<Node<U, T>>>, Rc<RefCell<Node<U, T>>>)) -> Self {
        Self {
            left: nodes.0,
            right: nodes.1,
            data: T::default(),
        }
    }
}

impl<T: Clone, U> From<&DirectedEdge<T, U>> for UndirectedEdge<T, U> {
    fn from(edge: &DirectedEdge<T, U>) -> Self {
        Self {
            left: edge.from.clone(),
            right: edge.to.clone(),
            data: edge.data.clone(),
        }
    }
}

impl<T, U> UndirectedEdge<T, U> {
    pub fn new(left: Rc<RefCell<Node<U, T>>>, right: Rc<RefCell<Node<U, T>>>, data: T) -> Self {
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
        let edge = Rc::new(RefCell::new(super::DirectedEdge::<Option<usize>, usize>::new(
            node_left.clone(),
            node_right.clone(),
            None,
        )));
        node_left.borrow_mut().edges.push(edge.clone());
        println!("{:?}", edge);
        println!("{:?}", node_left);
        let edge = Rc::new(RefCell::new(
            super::DirectedEdge::<Option<usize>, usize>::from((
                node_left.clone(),
                node_right.clone(),
            )),
        ));
        println!("{:?}", edge);
        println!("{:?}", node_left);
    }
}
