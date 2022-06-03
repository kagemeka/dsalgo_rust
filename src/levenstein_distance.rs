pub fn levenstein_distance<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let n: usize = a.len();
    let m: usize = b.len();
    let mut dist = vec![vec![n + m; m + 1]; n + 1];
    // at most n + m by removing all the a and inserting all the b.
    for i in 0..=n {
        dist[i][0] = i;
    }
    for j in 0..=m {
        dist[0][j] = j;
    }
    for i in 0..n {
        for j in 0..m {
            if a[i] == b[j] {
                dist[i + 1][j + 1] = dist[i][j];
                continue;
                // it's not necesary to check dist[i][j + 1] and dist[i + 1][j].
                // because dist[i][j] - 1 <= dist[i][j + 1], dist[i + 1][j].
                // (easy to prove with mathematical induction.)
            }
            dist[i + 1][j + 1] = [
                dist[i][j + 1],
                dist[i + 1][j],
                dist[i][j],
            ]
            .iter()
            .min()
            .unwrap()
                + 1;
        }
    }

    dist[n][m]
}
