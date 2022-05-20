#[derive(Debug)]
pub struct AdjacencyList<T> {
    pub(crate) edges: Vec<Vec<(usize, T)>>,
}

impl<E> AdjacencyList<E> {
    pub fn size(&self) -> usize { self.edges.len() }

    pub fn new(size: usize) -> Self {
        Self {
            edges: (0..size).map(|_| Vec::new()).collect(),
        }
    }

    pub fn add_node(&mut self) { self.edges.push(Vec::new()); }
}

impl<T> std::ops::Index<usize> for AdjacencyList<T> {
    type Output = Vec<(usize, T)>;

    fn index(&self, i: usize) -> &Self::Output { &self.edges[i] }
}

impl<T> std::ops::IndexMut<usize> for AdjacencyList<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.edges[i]
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
