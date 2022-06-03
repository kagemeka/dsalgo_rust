#[allow(dead_code)]
pub(crate) fn lcs_length_low_memory<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let m = b.len();
    let mut length = vec![0; m + 1];
    for x in a {
        for j in (0..m).rev() {
            if x == &b[j] {
                debug_assert!(length[j + 1] <= length[j] + 1);
                length[j + 1] = length[j] + 1;
            }
        }
        for j in 0..m {
            length[j + 1] = std::cmp::max(length[j], length[j + 1]);
        }
    }
    length[m]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let s = "axyb".chars().collect::<Vec<_>>();
        let t = "abyxb".chars().collect::<Vec<_>>();
        assert_eq!(
            lcs_length_low_memory(&s, &t),
            3
        );
    }
}
