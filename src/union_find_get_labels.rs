use crate::union_find::UnionFind;

impl UnionFind {
    pub fn get_labels(&mut self) -> Vec<usize> {
        let n = self.size();
        let mut labels = vec![n; n];
        let mut label = 0;
        for i in 0..n {
            let root = self.find_root(i);
            if labels[root] == n {
                labels[root] = label;
                label += 1;
            }
            labels[i] = labels[root];
        }
        labels
    }
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
