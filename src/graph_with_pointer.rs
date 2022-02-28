use std::{cell::RefCell, rc::Rc};

pub struct EdgeData;
pub struct NodeData;

pub trait Edge<T = Option<EdgeData>, U = Option<NodeData>> {}

impl<T, U> std::fmt::Debug for dyn Edge<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Edge") }
}

pub struct Node<T, U> {
    pub edges: Vec<Rc<RefCell<dyn Edge<U, T>>>>,
    pub data: T,
}

impl<T: std::fmt::Debug, U> std::fmt::Debug for Node<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ data: {:?}, edegs: {:?}}}", self.data, self.edges)
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

#[derive(Debug)]
pub struct Graph<T, U> {
    pub nodes: Vec<Rc<RefCell<Node<T, U>>>>,
}

impl<T, U> Graph<T, U> {
    pub fn size(&self) -> usize { self.nodes.len() }

    pub fn new(size: usize) -> Self
    where
        T: Default,
    {
        Self {
            nodes: vec![Rc::new(RefCell::new(Node::default())); size],
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
            .push(Rc::new(RefCell::new(DirectedEdge::new(
                self.nodes[from].clone(),
                self.nodes[to].clone(),
                data,
            ))));
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        use std::{cell::RefCell, rc::Rc};

        #[derive(Debug, Default, Clone)]
        struct PureNone;

        let node_left = Rc::new(RefCell::new(super::Node::default()));
        let node_right = Rc::new(RefCell::new(super::Node::default()));
        let edge = Rc::new(RefCell::new(super::DirectedEdge::<PureNone, usize>::new(
            node_left.clone(),
            node_right.clone(),
            PureNone,
        )));
        println!("{:?}", edge);
        println!("{:?}", node_left);
        node_left.borrow_mut().edges.push(edge.clone());
        println!("{:?}", edge);
        println!("{:?}", node_left);
        let edge = Rc::new(RefCell::new(super::DirectedEdge::<PureNone, usize>::from((
            node_left.clone(),
            node_right.clone(),
        ))));
        println!("{:?}", edge);
        println!("{:?}", node_left);

        let mut graph = super::Graph::<PureNone, usize>::new(2);
        println!("{:?}", graph);
        graph.add_directed_edge(0, 1, 3);
        println!("{:?}", graph);
    }
}
