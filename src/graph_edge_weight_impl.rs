use crate::graph_edge_trait::Weight;

impl Weight<Self> for u64 {
    fn weight(&self) -> &Self { self }
}

impl Weight<Self> for i64 {
    fn weight(&self) -> &Self { self }
}
