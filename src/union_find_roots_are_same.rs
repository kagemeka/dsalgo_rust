use crate::union_find::UnionFind;

impl UnionFind {
    pub fn are_same(&mut self, u: usize, v: usize) -> bool {
        self.find_root(u) == self.find_root(v)
    }
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
