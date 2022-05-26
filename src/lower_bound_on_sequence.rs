use crate::binary_search_on_sequence::binary_search;

pub fn lower_bound<T: PartialOrd>(monotonic_sequence: &[T], x: &T) -> usize {
    binary_search(
        &|y: &T| y >= x,
        monotonic_sequence,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let v = (0..10).collect::<Vec<_>>();

        assert_eq!(lower_bound(&v, &-1), 0);
        assert_eq!(lower_bound(&v, &0), 0);
        assert_eq!(lower_bound(&v, &15), 10);
    }
}
