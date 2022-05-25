use crate::lowlink::LowlinkResult;

pub fn undirected_lowlink(
    v_size: usize,
    undirected_edges: &[(usize, usize)],
) -> LowlinkResult {
    let mut g = vec![vec![]; v_size];
    for (i, &(u, v)) in undirected_edges.iter().enumerate() {
        g[u].push((v, i));
        g[v].push((u, i));
    }

    let mut orders = vec![v_size; v_size];
    let mut order = 0;
    let mut low_orders = vec![v_size; v_size];

    fn compute_low_order(
        g: &[Vec<(usize, usize)>],
        orders: &mut [usize],
        order: &mut usize,
        low_orders: &mut [usize],
        u: usize,
        edge_from: usize,
    ) {
        orders[u] = *order;
        low_orders[u] = *order;
        *order += 1;
        for &(v, eid) in &g[u] {
            if orders[v] == g.len() {
                compute_low_order(
                    g, orders, order, low_orders, v, eid,
                );
                low_orders[u] = std::cmp::min(low_orders[u], low_orders[v]);
            } else if eid != edge_from {
                low_orders[u] = std::cmp::min(low_orders[u], orders[v]);
            }
        }
    }

    for u in 0..v_size {
        if orders[u] == g.len() {
            compute_low_order(
                &g,
                &mut orders,
                &mut order,
                &mut low_orders,
                u,
                undirected_edges.len(),
            );
        }
    }

    LowlinkResult { orders, low_orders }
}

// TOO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
