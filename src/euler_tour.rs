pub fn euler_tour_edge(
    g: &Vec<(usize, usize)>,
    root: usize,
) -> (Vec<isize>, Vec<usize>, Vec<usize>) {
    let n = g.len() + 1;
    let mut t = vec![vec![]; n];
    for &(u, v) in g.iter() {
        t[u].push(v);
        t[v].push(u);
    }
    let mut parent = vec![n; n];
    let mut depth = vec![0; n];
    let mut tour = Vec::with_capacity(n << 1);
    let mut st = vec![root as isize];
    for _ in 0..n << 1 {
        let u = st.pop().unwrap();
        tour.push(u);
        if u < 0 {
            continue;
        }
        st.push(!u);
        let u = u as usize;
        for &v in t[u].iter() {
            if v == parent[u] {
                continue;
            }
            parent[v] = u;
            depth[v] = depth[u] + 1;
        }
    }
    (tour, parent, depth)
}

pub fn euler_tour_node(
    g: &Vec<(usize, usize)>,
    root: usize,
) -> (Vec<isize>, Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
    let (mut tour, parent, depth) = euler_tour_edge(g, root);
    let n = tour.len() >> 1;
    tour.pop();
    let mut first_idx = vec![n; n];
    let mut last_idx = vec![n; n];
    for i in 0..tour.len() {
        let mut u = tour[i];
        if u < 0 {
            u = parent[!u as usize] as isize;
            tour[i] = u;
        } else {
            first_idx[u as usize] = i;
        }
        last_idx[u as usize] = i;
    }
    (tour, first_idx, last_idx, parent, depth)
}

pub mod extension {
    pub fn compute_first_idx(tour_edge: &Vec<isize>) -> Vec<usize> {
        let n = tour_edge.len() >> 1;
        let mut first_idx = vec![0; n];
        for (i, &u) in tour_edge.iter().enumerate() {
            if u >= 0 {
                first_idx[u as usize] = i;
            }
        }
        first_idx
    }

    // pub fn compute_last_idx(tour_edge: &Vec<isize>) ->
    // Vec<usize> {     let n = tour_edge.len() >> 1;
    //     let mut last_idx = vec![0; n];
    //     for (i, &u) in tour_edge.iter().enumerate() {
    //         if u < 0 { last_idx[u as usize] = i; }
    //     }
    //     last_idx
    // }
}
