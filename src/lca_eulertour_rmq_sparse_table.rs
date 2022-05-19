use crate::{
    group_theory_id::Min,
    lca_eulertour_rmq::LCAEulerTourRMQ,
    sparse_table::SparseTable,
};

#[allow(dead_code)]
type LCAEulerTourRMQSparseTable =
    LCAEulerTourRMQ<SparseTable<(usize, usize), Min>>;
