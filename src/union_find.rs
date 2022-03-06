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

pub struct PotentialUnionFind<
    G: crate::group_theory::AbelianGroup<S, T>,
    S = G,
    T = crate::group_theory::Additive,
> {
    phantom_g: std::marker::PhantomData<G>,
    phantom_t: std::marker::PhantomData<T>,
    data: Vec<isize>,
    potential_from_parent: Vec<S>,
}

impl<G: crate::group_theory::AbelianGroup<S, T>, S, T> PotentialUnionFind<G, S, T> {
    pub fn new(size: usize) -> Self
    where
        S: Clone,
    {
        Self {
            phantom_g: std::marker::PhantomData,
            phantom_t: std::marker::PhantomData,
            data: vec![-1; size],
            potential_from_parent: vec![G::identity(); size],
        }
    }

    pub fn size(&self) -> usize { self.data.len() }

    pub fn find_root(&mut self, node: usize) -> usize {
        assert!(node < self.size());
        if self.data[node] < 0 {
            return node;
        }
        let parent = self.data[node] as usize;
        self.data[node] = self.find_root(parent) as isize;
        self.potential_from_parent[node] = G::operate(
            &self.potential_from_parent[node],
            &self.potential_from_parent[parent],
        );
        self.data[node] as usize
    }

    pub fn potential(&mut self, node: usize) -> S {
        self.find_root(node);
        G::operate(&G::identity(), &self.potential_from_parent[node])
    }

    pub fn unite(
        &mut self,
        mut left_node: usize,
        mut right_node: usize,
        potential_left_to_right: &S,
    ) where
        S: PartialEq + std::fmt::Debug,
    {
        assert!(left_node < self.size() && right_node < self.size());
        let mut potential_left_to_right = G::operate(
            &G::operate(&self.potential_from_parent[left_node], potential_left_to_right),
            &G::invert(&self.potential_from_parent[right_node]),
        );
        left_node = self.find_root(left_node);
        right_node = self.find_root(right_node);
        if left_node == right_node {
            assert_eq!(potential_left_to_right, G::identity());
            return;
        }
        if self.data[left_node] > self.data[right_node] {
            (left_node, right_node) = (right_node, left_node);
            potential_left_to_right = G::invert(&potential_left_to_right);
        }
        self.data[left_node] += self.data[right_node];
        self.data[right_node] = left_node as isize;
        self.potential_from_parent[right_node] = potential_left_to_right;
    }

    pub fn size_of(&mut self, node: usize) -> usize {
        assert!(node < self.size());
        let root = self.find_root(node);
        -self.data[root] as usize
    }

    pub fn potential_difference(&mut self, from: usize, to: usize) -> Option<S> {
        if self.find_root(from) != self.find_root(to) {
            None
        } else {
            Some(G::operate(
                &G::invert(&self.potential(from)),
                &self.potential(to),
            ))
        }
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

    #[test]
    fn test_potential_union_find() {
        struct IntAdd;
        impl crate::group_theory::BinaryOperation<i32> for IntAdd {
            fn operate(lhs: &i32, rhs: &i32) -> i32 { lhs + rhs }
        }
        impl crate::group_theory::Associative<i32> for IntAdd {}
        impl crate::group_theory::Commutative<i32> for IntAdd {}
        impl crate::group_theory::Inverse<i32> for IntAdd {
            fn invert(element: &i32) -> i32 { -element }
        }
        impl crate::group_theory::Identity<i32> for IntAdd {
            fn identity() -> i32 { 0 }
        }

        let mut uf = super::PotentialUnionFind::<IntAdd, i32>::new(10);
        assert_eq!(uf.size_of(0), 1);
        uf.unite(3, 9, &5);
        assert_eq!(uf.size_of(3), 2);
        assert_eq!(uf.potential_difference(3, 9), Some(5));
        assert_eq!(uf.potential_difference(1, 3), None);
    }
}
