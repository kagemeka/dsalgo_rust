pub fn pascal_triangle<T>(size: usize) -> Vec<Vec<T>>
where
    T: std::ops::Add<Output = T> + From<u64> + Clone,
{
    let mut p = (0..size)
        .map(|i| vec![1.into(); i + 1])
        .collect::<Vec<Vec<T>>>();
    for i in 1..size {
        for j in 1..i {
            p[i][j] = p[i - 1][j - 1].clone() + p[i - 1][j].clone();
        }
        // p[i][i] = 1 is already initialized.
    }
    p
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let p = pascal_triangle::<u64>(6);
        assert_eq!(
            p,
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
                vec![1, 5, 10, 10, 5, 1],
            ]
        );
    }
}
