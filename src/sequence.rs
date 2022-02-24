use crate::{binary_search::lower_bound, cmp::Infinity};

pub fn longest_increasing_sequence<T: Ord + Infinity + Clone + Copy>(
    a: &[T],
) -> Vec<T> {
    let n = a.len();
    let mut lis = vec![T::INFINITY; n];
    for &x in a.iter() {
        let i = lower_bound(&lis, &x);
        lis[i] = x;
    }
    let i = lower_bound(&lis, &T::INFINITY);
    lis[..i].to_vec()
}

// pub fn longest_non_decreasing_sequence<T: Ord + Infinity +
// Clone + Copy>(     arr: &[T],
// ) -> Vec<T> {
// }
