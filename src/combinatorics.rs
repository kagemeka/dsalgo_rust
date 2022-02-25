use crate::abstract_traits;

pub trait PascalTriangle<S = Self, T = abstract_traits::Additive>:
    abstract_traits::Monoid<S, T> + abstract_traits::Default<S, T>
{
    fn pascal_triangle(n: usize) -> Vec<Vec<S>>;
}

impl<S, T, U> PascalTriangle<S, T> for U
where
    U: abstract_traits::Monoid<S, T> + abstract_traits::Default<S, T>,
    S: Clone,
{
    fn pascal_triangle(n: usize) -> Vec<Vec<S>> {
        let mut p: Vec<Vec<S>> = vec![vec![U::identity(); n]; n];
        for i in 0..n {
            p[i][0] = U::default();
        }
        for i in 1..n {
            for j in 1..i + 1 {
                p[i][j] = U::operate(&p[i - 1][j], &p[i - 1][j - 1]);
            }
        }
        p
    }
}
