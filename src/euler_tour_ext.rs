pub fn last_positions(tour_nodes: &[usize]) -> Vec<usize> {
    let n = tour_nodes.iter().max().unwrap() + 1;
    let mut pos = vec![None; n];
    tour_nodes
        .iter()
        .enumerate()
        .for_each(|(i, &u)| pos[u] = Some(i));
    pos.iter().map(|&p| p.unwrap()).collect()
}

pub fn first_positions(tour_nodes: &[usize]) -> Vec<usize> {
    let size = tour_nodes.len();
    let mut tour = tour_nodes.to_vec();
    tour.reverse();
    last_positions(&tour).iter().map(|&i| size - i - 1).collect()
}
