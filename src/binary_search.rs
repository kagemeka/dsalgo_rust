pub fn binary_search<T, F>(is_ok: F, slice: &[T]) -> usize
where
    F: Fn(&T) -> bool,
{
    let mut low = 0usize;
    let mut high = slice.len();
    while low < high {
        let i = (low + high - 1) >> 1;
        if is_ok(&slice[i]) {
            high = i;
        } else {
            low = i + 1;
        }
    }
    high
}

pub fn lower_bound<T: PartialOrd>(slice: &[T], x: &T) -> usize {
    binary_search(&|y: &T| y >= x, slice)
}

pub fn upper_bound<T: PartialOrd>(slice: &[T], x: &T) -> usize {
    binary_search(&|y: &T| y > x, slice)
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_search() {
        let v = (0..10).collect::<Vec<_>>();
        assert_eq!(super::binary_search(&|x: &i32| x >= &5, &v), 5);
        assert_eq!(super::binary_search(&|x: &i32| x >= &10, &v), 10);
        assert_eq!(super::binary_search(&|x: &i32| x >= &11, &v), 10);
        assert_eq!(super::lower_bound(&v, &-1), 0);
        assert_eq!(super::upper_bound(&v, &-1), 0);
        assert_eq!(super::lower_bound(&v, &0), 0);
        assert_eq!(super::upper_bound(&v, &0), 1);
        assert_eq!(super::lower_bound(&v, &15), 10);
        assert_eq!(super::upper_bound(&v, &15), 10);
    }
}
