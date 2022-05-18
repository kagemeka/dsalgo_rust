pub fn solve_ghost_leg(n: usize, edges: &[usize]) -> Vec<usize> {
    let mut res = (0..n).collect::<Vec<_>>();
    for &i in edges.iter().rev() {
        res.swap(i, i + 1);
    }
    res
}
