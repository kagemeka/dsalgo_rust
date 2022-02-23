use crate::{binary_search::lower_bound, cmp::Inf};

pub fn longest_increasing_sequence<T: Ord + Inf + Clone + Copy>(
    a: &[T],
) -> Vec<T> {
    let n = a.len();
    let mut lis = vec![T::INF; n];
    for &x in a.iter() {
        let i = lower_bound(&lis, &x);
        lis[i] = x;
    }
    let i = lower_bound(&lis, &T::INF);
    lis[..i].to_vec()
}
