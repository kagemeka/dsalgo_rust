use crate::graph_edge_trait::From;

#[derive(Debug)]
pub struct AdjacencyList<E> {
    pub(crate) edges: Vec<Vec<E>>,
}

impl<E> AdjacencyList<E> {
    pub fn size(&self) -> usize { self.edges.len() }

    pub fn edges(&self) -> &[Vec<E>] { &self.edges }

    pub fn new(size: usize) -> Self {
        Self {
            edges: (0..size).map(|_| Vec::new()).collect(),
        }
    }

    pub fn add_node(&mut self) { self.edges.push(Vec::new()); }
}

impl<E> std::ops::Index<usize> for AdjacencyList<E> {
    type Output = Vec<E>;

    fn index(&self, i: usize) -> &Self::Output { &self.edges[i] }
}

impl<T> std::ops::IndexMut<usize> for AdjacencyList<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.edges[i]
    }
}

impl<E> std::convert::From<(usize, Vec<E>)> for AdjacencyList<E>
where
    E: From<V = usize>,
{
    fn from((size, edges): (usize, Vec<E>)) -> Self {
        let mut g = Self::new(size);
        for e in edges.into_iter() {
            g[*e.from()].push(e);
        }
        g
    }
}

impl<E> std::convert::Into<Vec<E>> for AdjacencyList<E>
where
    E: From<V = usize>,
{
    fn into(self) -> Vec<E> {
        self.edges
            .into_iter()
            .enumerate()
            .flat_map(|(u, edges)| {
                assert!(edges.iter().all(|e| e.from() == &u));
                edges
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug_print::debug_print;
    #[test]
    fn test() {
        let mut g = AdjacencyList::new(5);
        g[0].push((1, 1));
        debug_print(&g);
    }
}
