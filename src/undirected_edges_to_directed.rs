use crate::graph_edge_trait::{Reversed, ToDirected};

pub fn edges_to_directed<UE, DE>(undirected_edges: Vec<UE>) -> Vec<DE>
where
    UE: Clone + ToDirected<E = DE>,
    DE: Reversed,
{
    let di_edges = undirected_edges.into_iter().map(|e| e.to_directed());
    let rev = di_edges.clone().map(|e| e.reversed());
    di_edges.chain(rev).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // TODO:
    }
}
