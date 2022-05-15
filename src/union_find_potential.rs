pub struct PotentialUnionFind<
    S: crate::group_theory::AbelianGroup<I> + Copy,
    I: crate::group_theory::BinaryOperationIdentifier,
> {
    phantom_i: std::marker::PhantomData<I>,
    data: Vec<isize>,
    potential_from_parent: Vec<S>,
}

impl<S, I> PotentialUnionFind<S, I>
where
    S: crate::group_theory::AbelianGroup<I> + Copy,
    I: crate::group_theory::BinaryOperationIdentifier,
{
    pub fn new(size: usize) -> Self
    where
        S: Clone,
    {
        Self {
            phantom_i: std::marker::PhantomData,
            data: vec![-1; size],
            potential_from_parent: vec![S::identity(); size],
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
        // self.potential_from_parent[node] = S::operate(
        //     &self.potential_from_parent[node],
        //     &self.potential_from_parent[parent],
        // );
        self.potential_from_parent[node] = self.potential_from_parent[node]
            .operate(self.potential_from_parent[parent]);
        self.data[node] as usize
    }

    pub fn potential(&mut self, node: usize) -> S {
        self.find_root(node);
        self.potential_from_parent[node]
        // S::operate(&S::identity(),
        // self.potential_from_parent[node])
    }

    pub fn unite(
        &mut self,
        mut left_node: usize,
        mut right_node: usize,
        potential_left_to_right: S,
    ) where
        S: PartialEq + std::fmt::Debug,
    {
        assert!(left_node < self.size() && right_node < self.size());
        // let mut potential_left_to_right = S::operate(
        //     &S::operate(&self.potential_from_parent[left_node],
        // potential_left_to_right),     &S::invert(&self.
        // potential_from_parent[right_node]), );
        let mut potential_left_to_right = self.potential_from_parent[left_node]
            .operate(potential_left_to_right)
            .operate(self.potential_from_parent[right_node].invert());
        left_node = self.find_root(left_node);
        right_node = self.find_root(right_node);
        if left_node == right_node {
            assert_eq!(
                potential_left_to_right,
                S::identity()
            );
            return;
        }
        if self.data[left_node] > self.data[right_node] {
            (left_node, right_node) = (right_node, left_node);
            // potential_left_to_right =
            // S::invert(&potential_left_to_right);
            potential_left_to_right = potential_left_to_right.invert();
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

    pub fn potential_difference(
        &mut self,
        from: usize,
        to: usize,
    ) -> Option<S> {
        if self.find_root(from) != self.find_root(to) {
            None
        } else {
            Some(self.potential(from).invert().operate(self.potential(to)))
            // Some(S::operate(
            //     &S::invert(&self.potential(from)),
            //     &self.potential(to),
            // ))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_potential_union_find() {
        use crate::group_theory::Additive;
        let mut uf = super::PotentialUnionFind::<i32, Additive>::new(10);
        assert_eq!(uf.size_of(0), 1);
        // uf.unite(3, 9, &5);
        uf.unite(3, 9, 5);
        assert_eq!(uf.size_of(3), 2);
        assert_eq!(
            uf.potential_difference(3, 9),
            Some(5)
        );
        assert_eq!(
            uf.potential_difference(1, 3),
            None
        );
    }
}
