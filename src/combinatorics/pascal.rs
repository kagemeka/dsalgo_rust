use crate::algebra::abstract_::structure::traits;

pub fn pascal<T: Copy + traits::Semiring>(n: usize) -> Vec<Vec<T>> {
    let mut p: Vec<Vec<T>> = vec![vec![<T as traits::AddIdentity>::e(); n]; n];
    for i in 0..n {
        p[i][0] = <T as traits::MulIdentity>::e();
    }
    for i in 1..n {
        for j in 1..i + 1 {
            p[i][j] = p[i - 1][j] + p[i - 1][j - 1];
        }
    }
    p
}
