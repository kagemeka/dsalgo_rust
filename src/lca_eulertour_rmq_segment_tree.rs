use crate::{
    group_theory_id::Min,
    lca_eulertour_rmq::LCAEulerTourRMQ,
    segment_tree::SegmentTree,
};

#[allow(dead_code)]
type LCAEulerTourRMQSegTree = LCAEulerTourRMQ<SegmentTree<(usize, usize), Min>>;
