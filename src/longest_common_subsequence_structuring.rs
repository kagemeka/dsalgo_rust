use crate::{
    longest_common_subsequence_dp_table::lcs_dp_table,
    longest_common_subsequence_restore_indices::restore_lcs_indices,
};

pub fn struct_lcs<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    restore_lcs_indices(&lcs_dp_table(a, b))
        .into_iter()
        .map(|(i, _)| a[i].clone())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let s = "axyb".chars().collect::<Vec<_>>();
        let t = "abyxb".chars().collect::<Vec<_>>();
        assert_eq!(
            struct_lcs(&s, &t),
            vec!['a', 'y', 'b']
        );
    }
}
