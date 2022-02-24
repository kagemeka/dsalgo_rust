use crate::abstract_traits;

pub fn pascal_triangle<T, S>(n: usize) -> Vec<Vec<S>>
where
    S: Clone + abstract_traits::Monoid<T> + abstract_traits::Default<T>,
{
    let mut p: Vec<Vec<S>> = vec![vec![S::identity(); n]; n];
    for i in 0..n {
        p[i][0] = S::default();
    }
    for i in 1..n {
        for j in 1..i + 1 {
            p[i][j] = S::operate(&p[i - 1][j], &p[i - 1][j - 1]);
        }
    }
    p
}
