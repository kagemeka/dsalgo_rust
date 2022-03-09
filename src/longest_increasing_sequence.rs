pub(self) fn find_longest_sequence<T: Copy, F: Fn(&T, &T) -> bool>(
    slice: &[T],
    binary_relation: F,
) -> Vec<T> {
    let mut result = vec![None; slice.len()];
    for &value in slice {
        let is_ok = |x: &Option<T>| {
            if let Some(x) = x {
                binary_relation(x, &value)
            } else {
                true
            }
        };
        let index = crate::binary_search::binary_search(&is_ok, &result);
        result[index] = Some(value);
    }
    let index = crate::binary_search::binary_search(|value: &Option<T>| value.is_none(), &result);
    result[..index].iter().map(|x| x.unwrap()).collect()
}

pub fn longest_increasing_sequence<T: PartialOrd + Clone + Copy>(slice: &[T]) -> Vec<T> {
    find_longest_sequence(slice, |x, value| x >= value)
}

pub fn longest_non_decreasing_sequence<T: PartialOrd + Clone + Copy>(slice: &[T]) -> Vec<T> {
    find_longest_sequence(slice, |x, value| x > value)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let arr = [4, 2, 8, 5, 6, 6];
        assert_eq!(super::longest_increasing_sequence(&arr), vec![2, 5, 6]);
        assert_eq!(super::longest_non_decreasing_sequence(&arr), vec![2, 5, 6, 6]);
    }
}
