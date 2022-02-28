// use crate::graph_with_pointer::{EdgeData, NodeData};

pub struct DirectedEdge<T> {
    pub from: usize,
    pub to: usize,
    pub data: T,
}

pub struct DirectedGraph<T, U> {
    pub node_datas: Vec<T>,
    pub edges: Vec<Vec<DirectedEdge<U>>>,
}

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_option_default() {
        let a = Option::<usize>::default();
        println!("{}", a.is_none());
    }

    #[test]
    fn test() {
        #[derive(Clone)]
        struct PhantomData;

        let graph = super::DenseGraph::<Option<PhantomData>, usize>::new(3);
        // assert_eq!(super::DenseGraph<usize>,
        // super::DenseGraph2<usize>);
    }
}
