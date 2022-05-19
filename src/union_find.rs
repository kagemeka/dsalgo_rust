#[derive(Debug)]
pub struct UnionFind {
    data: Vec<isize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self { Self { data: vec![-1; size] } }

    pub fn size(&self) -> usize { self.data.len() }

    pub fn find_root(&mut self, u: usize) -> usize {
        if self.data[u] < 0 {
            return u;
        }
        self.data[u] = self.find_root(self.data[u] as usize) as isize;
        self.data[u] as usize
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        let mut u = self.find_root(u);
        let mut v = self.find_root(v);
        if u == v {
            return;
        }
        if self.data[u] > self.data[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.data[u] += self.data[v];
        self.data[v] = u as isize;
    }

    pub fn size_of(&mut self, u: usize) -> usize {
        let u = self.find_root(u);
        -self.data[u] as usize
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;
    #[test]
    fn test_uf() {
        let mut uf = UnionFind::new(10);
        assert_eq!(uf.size_of(0), 1);
        uf.unite(3, 9);
        assert_eq!(uf.size_of(3), 2);
    }
}
