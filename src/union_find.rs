pub struct UnionFind {
    data: Vec<i32>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![-1; n],
        }
    }

    pub fn find(&mut self, u: usize) -> usize {
        if self.data[u] < 0 {
            return u;
        }
        self.data[u] = self.find(self.data[u] as usize) as i32;
        self.data[u] as usize
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        let (mut u, mut v) = (self.find(u), self.find(v));
        if u == v {
            return;
        }
        if self.data[u] > self.data[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.data[u] += self.data[v];
        self.data[v] = u as i32;
    }

    pub fn size(&mut self, u: usize) -> usize {
        let u = self.find(u);
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
        assert_eq!(uf.size(0), 1);
        uf.unite(3, 9);
        assert_eq!(uf.size(3), 2);
    }
}
