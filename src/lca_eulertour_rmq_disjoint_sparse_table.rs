use crate::{
    disjoint_sparse_table::DisjointSparseTable,
    group_theory_id::Min,
    lca_eulertour_rmq::LCAEulerTourRMQ,
};

#[allow(dead_code)]
type LCAEulerTourRMQDisjointSparseTable =
    LCAEulerTourRMQ<DisjointSparseTable<(usize, usize), Min>>;
