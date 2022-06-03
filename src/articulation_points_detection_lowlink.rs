use crate::undirected_lowlink::undirected_lowlink;

/// return vertex ids.
pub fn find_articulation_points_lowlink(
    v_size: usize,
    undirected_edges: &[(usize, usize)],
) -> Vec<usize> {
    let mut articulation_points = Vec::new();
    let lowlink = undirected_lowlink(v_size, undirected_edges);
    let order = lowlink.orders;
    let low = lowlink.low_orders;

    let mut visited = vec![false; v_size];

    let mut g = vec![vec![]; v_size];
    for &(u, v) in undirected_edges {
        g[u].push(v);
        g[v].push(u);
    }
    fn find(
        g: &[Vec<usize>],
        order: &[usize],
        low: &[usize],
        points: &mut Vec<usize>,
        visited: &mut Vec<bool>,
        u: usize,
        parent: usize,
    ) {
        let n = g.len();
        visited[u] = true;
        let mut childs_count = 0;
        let mut is_articulation = false;
        for &v in &g[u] {
            if visited[v] {
                continue;
            }
            childs_count += 1;
            find(
                g, order, low, points, visited, v, u,
            );
            if parent != n && low[v] >= order[u] {
                is_articulation = true;
            }
        }
        if parent == n && childs_count > 1 {
            is_articulation = true;
        }
        if is_articulation {
            points.push(u);
        }
    }
    for i in 0..v_size {
        if visited[i] {
            continue;
        }
        find(
            &g,
            &order,
            &low,
            &mut articulation_points,
            &mut visited,
            i,
            v_size,
        );
    }
    articulation_points
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
