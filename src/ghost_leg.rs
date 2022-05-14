pub fn solve_ghost_leg(n: usize, edges: Vec<usize>) -> Result<Vec<usize>, String> {
    assert!(n > 0);
    let mut res = (0..n).collect::<Vec<_>>();
    for &i in edges.iter().rev() {
        if i >= n - 1 {
            return Err(format!("invalid edge index: {}", i));
        }
        res.swap(i, i + 1);
    }
    Ok(res)
}
