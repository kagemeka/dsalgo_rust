use crate::abstract_traits;

pub fn pascal_triangle<T: Copy + abstract_traits::Semiring>(
    n: usize,
) -> Vec<Vec<T>> {
    let mut p: Vec<Vec<T>> =
        vec![vec![<T as abstract_traits::AddIdentity>::e(); n]; n];
    for i in 0..n {
        p[i][0] = <T as abstract_traits::MulIdentity>::e();
    }
    for i in 1..n {
        for j in 1..i + 1 {
            p[i][j] = p[i - 1][j] + p[i - 1][j - 1];
        }
    }
    p
}
