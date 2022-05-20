#[derive(Debug)]
pub struct AdjacencyMatrixGraph<E> {
    matrix: Vec<Vec<Option<E>>>,
}

impl<E> AdjacencyMatrixGraph<E> {
    pub fn size(&self) -> usize { self.matrix.len() }

    pub fn new(size: usize) -> Self {
        Self {
            matrix: (0..size)
                .map(|_| (0..size).map(|_| None).collect())
                .collect(),
        }
    }

    pub fn resize(&mut self, size: usize) {
        while self.size() > size {
            self.matrix.pop();
        }
        while self.size() < size {
            self.matrix.push((0..size).map(|_| None).collect());
        }
        for i in 0..self.size() {
            while self.matrix[i].len() > size {
                self.matrix[i].pop();
            }
            while self.matrix[i].len() < size {
                self.matrix[i].push(None);
            }
        }
    }

    pub fn set(&mut self, u: usize, v: usize, data: E) {
        self.matrix[u][v] = Some(data);
    }

    pub fn get(&self, u: usize, v: usize) -> Option<&E> {
        self.matrix[u][v].as_ref()
    }
}

impl<E: std::fmt::Debug> AdjacencyMatrixGraph<E> {
    pub fn print_debug(&self) {
        println!("{:#?}", self);
    }
}
