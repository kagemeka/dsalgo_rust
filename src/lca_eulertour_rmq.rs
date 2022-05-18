use crate::{
    euler_tour_ext::first_positions,
    euler_tour_nodes::euler_tour_nodes,
    range_minimum_query::RangeMinimumQuery,
    tree_depths::tree_depths,
};

pub struct LCAEulerTourRMQ<Q> {
    first_pos: Vec<usize>,
    rmq: Q,
}

impl<Q> LCAEulerTourRMQ<Q> {
    pub fn new(tree_edges: &[(usize, usize)], root: usize) -> Self
    where
        Q: std::iter::FromIterator<(usize, usize)>,
    {
        let tour_nodes = euler_tour_nodes(tree_edges, root);
        let depth = tree_depths(tree_edges, root);
        let first_pos = first_positions(&tour_nodes);
        let depth = tour_nodes.iter().map(|&u| depth[u]).collect::<Vec<_>>();
        let rmq = Q::from_iter(depth.into_iter().zip(tour_nodes.into_iter()));
        Self { first_pos, rmq }
    }

    pub fn get(&mut self, u: usize, v: usize) -> usize
    where
        Q: RangeMinimumQuery<(usize, usize)>,
    {
        let mut left = self.first_pos[u];
        let mut right = self.first_pos[v];
        if left > right {
            std::mem::swap(&mut left, &mut right);
        }
        self.rmq.range_minimum(left, right + 1).1
    }
}
