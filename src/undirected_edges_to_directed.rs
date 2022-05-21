use crate::graph_edge_trait::{Reversed, ToDirected};

pub fn edges_to_directed<E>(undirected_edges: Vec<E>) -> Vec<E>
where
    E: Clone + Reversed + ToDirected<E = E>,
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
