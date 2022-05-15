use crate::{
    group_theory_id::Min,
    lca_eulertour_rmq::LCAEulerTourRMQ,
    segment_tree::SegmentTree,
};

type LCAEulerTourRMQSegTree =
    LCAEulerTourRMQ<SegmentTree<(usize, usize), Min, (usize, usize)>>;
