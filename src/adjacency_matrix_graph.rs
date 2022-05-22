#[derive(Debug)]
pub struct AdjacencyMatrix<T> {
    pub(crate) data: Vec<Vec<T>>,
}

impl<T> AdjacencyMatrix<T> {
    pub fn size(&self) -> usize { self.data.len() }

    pub fn data(&self) -> &[Vec<T>] { &self.data }

    pub fn new(size: usize) -> Self
    where
        T: Default,
    {
        Self {
            data: (0..size)
                .map(|_| (0..size).map(|_| T::default()).collect())
                .collect(),
        }
    }

    pub fn add_node(&mut self)
    where
        T: Default,
    {
        self.data
            .push((0..self.size()).map(|_| T::default()).collect());
        for i in 0..self.size() {
            self.data[i].push(T::default());
        }
    }
}

impl<T> std::ops::Index<(usize, usize)> for AdjacencyMatrix<T> {
    type Output = T;

    fn index(&self, (u, v): (usize, usize)) -> &Self::Output {
        &self.data[u][v]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for AdjacencyMatrix<T> {
    fn index_mut(&mut self, (u, v): (usize, usize)) -> &mut Self::Output {
        &mut self.data[u][v]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug_print::debug_print;
    #[test]
    fn test() {
        let mut g = AdjacencyMatrix::new(5);
        g[(0, 0)] = 1;
        debug_print(&g);
    }
}
