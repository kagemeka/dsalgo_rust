pub struct UnionFind {
    data: Vec<isize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![-1; size],
        }
    }

    pub fn size(&self) -> usize { self.data.len() }

    pub fn find_root(&mut self, node: usize) -> usize {
        assert!(node < self.size());
        if self.data[node] < 0 {
            return node;
        }
        self.data[node] = self.find_root(self.data[node] as usize) as isize;
        self.data[node] as usize
    }

    pub fn unite(&mut self, left_node: usize, right_node: usize) {
        assert!(left_node < self.size() && right_node < self.size());
        let (mut u, mut v) = (self.find_root(left_node), self.find_root(right_node));
        if u == v {
            return;
        }
        if self.data[u] > self.data[v] {
            (u, v) = (v, u);
        }
        self.data[u] += self.data[v];
        self.data[v] = u as isize;
    }

    pub fn size_of(&mut self, u: usize) -> usize {
        let u = self.find_root(u);
        -self.data[u] as usize
    }
}

pub struct RollbackUnionFind {}

pub struct PersistentUnionFind {}

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
