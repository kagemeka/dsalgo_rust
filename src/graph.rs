// #[derive(Debug, Clone)]
// pub enum Edge<T> {
//     Directed {
//         from: usize,
//         to: usize,
//         data: T,
//     },
//     Undirected {
//         left: usize,
//         right: usize,
//         data: T,
//     },
// }

#[derive(Debug, Clone)]
pub struct DirectedEdge<T> {
    pub from: usize,
    pub to: usize,
    pub data: T,
}

#[derive(Debug, Clone)]
pub struct UndirectedEdge<T> {
    pub left: usize,
    pub right: usize,
    pub data: T,
}

#[derive(Debug)]
pub struct DirectedGraph<T = (), U = ()> {
    pub node_datas: Vec<T>,
    pub edges: Vec<Vec<DirectedEdge<U>>>,
}

impl<U: Clone> From<&[UndirectedEdge<U>]> for DirectedGraph<(), U> {
    fn from(edges: &[UndirectedEdge<U>]) -> Self {
        let mut graph = Self::new(edges.len() + 1);
        for edge in edges {
            graph.add_edge(edge.left, edge.right, edge.data.clone());
            graph.add_edge(edge.right, edge.left, edge.data.clone());
        }
        graph
    }
}

impl<T, U> DirectedGraph<T, U> {
    pub fn new(size: usize) -> Self
    where
        T: Clone + Default,
        U: Clone,
    {
        Self {
            node_datas: vec![T::default(); size],
            edges: vec![vec![]; size],
        }
    }

    pub fn size(&self) -> usize { self.node_datas.len() }

    pub fn add_node(&mut self)
    where
        T: Default,
    {
        self.node_datas.push(T::default());
        self.edges.push(vec![]);
    }

    pub fn add_edge(&mut self, from: usize, to: usize, data: U) {
        assert!(from < self.size() && to < self.size());
        self.edges[from].push(DirectedEdge { from, to, data });
    }
}

use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
pub struct UndirectedGraph<T = (), U = ()> {
    pub node_datas: Vec<T>,
    pub edges: Vec<Vec<Rc<RefCell<UndirectedEdge<U>>>>>,
}

impl<T, U> UndirectedGraph<T, U> {
    pub fn new(size: usize) -> Self
    where
        T: Clone + Default,
        U: Clone,
    {
        Self {
            node_datas: vec![T::default(); size],
            edges: vec![vec![]; size],
        }
    }

    pub fn size(&self) -> usize { self.node_datas.len() }

    pub fn add_node(&mut self)
    where
        T: Default,
    {
        self.node_datas.push(T::default());
        self.edges.push(vec![]);
    }

    pub fn add_edge(&mut self, left: usize, right: usize, data: U) {
        assert!(left < self.size() && right < self.size());
        let edge = Rc::new(RefCell::new(UndirectedEdge {
            left,
            right,
            data,
        }));
        self.edges[left].push(edge.clone());
        self.edges[right].push(edge.clone());
    }
}

#[derive(Debug)]
pub struct DenseGraph<T, U> {
    pub node_datas: Vec<T>,
    pub edge_datas: Vec<Vec<U>>,
}

impl<T, U> DenseGraph<T, U> {
    pub fn new(size: usize) -> Self
    where
        T: Default + Clone,
        U: Default + Clone,
    {
        Self {
            node_datas: vec![T::default(); size],
            edge_datas: vec![vec![U::default(); size]; size],
        }
    }

    pub fn size(&self) -> usize { self.node_datas.len() }

    pub fn add_node(&mut self)
    where
        T: Default,
        U: Default + Clone,
    {
        self.node_datas.push(T::default());
        for from in 0..self.size() {
            self.edge_datas[from].push(U::default());
        }
        self.edge_datas.push(vec![U::default(); self.size() + 1]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_option_default() {
        let a = Option::<usize>::default();
        println!("{}", a.is_none());
    }

    #[test]
    fn test_undirected() {
        #[derive(Clone, Debug)]
        struct Data {
            weight: isize,
        }
        let mut graph = super::UndirectedGraph::<(), Data>::new(3);
        graph.add_edge(1, 2, Data { weight: 1 });
        for from in 0..3 {
            for edge in graph.edges[from].iter() {
                println!("{:?}", edge);
                println!("{}", edge.borrow().data.weight);
            }
        }
    }

    #[test]
    fn test_directed() {
        let edges = (0..2)
            .map(|i| super::UndirectedEdge::<()> {
                left: i,
                right: i + 1,
                data: (),
            })
            .collect::<Vec<_>>();
        let graph = super::DirectedGraph::<(), ()>::from(edges.as_slice());
        println!("{:?}", graph);
    }

    #[test]
    fn test_dense() {
        let mut graph = super::DenseGraph::<(), usize>::new(3);
        graph.edge_datas[1][2] = 3;
        println!("{:?}", graph);
    }
}
