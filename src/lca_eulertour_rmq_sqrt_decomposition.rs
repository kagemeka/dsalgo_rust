use crate::{
    group_theory_id::Min,
    lca_eulertour_rmq::LCAEulerTourRMQ,
    sqrt_decomposition::SqrtDecomposition,
};

#[allow(dead_code)]
type LCAEulerTourRMQSqrtDecomposition =
    LCAEulerTourRMQ<SqrtDecomposition<(usize, usize), Min>>;
