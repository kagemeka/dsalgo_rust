pub fn pascal_triangle<T>(size: usize) -> Vec<Vec<T>>
where
    T: std::ops::Add<Output = T> + From<usize> + Copy,
{
    let mut p = vec![vec![0.into(); size]; size];
    for i in 0..size {
        p[i][0] = 1.into();
    }
    for i in 1..size {
        for j in 1..size {
            p[i][j] = p[i - 1][j - 1] + p[i - 1][j];
        }
    }
    p
}
