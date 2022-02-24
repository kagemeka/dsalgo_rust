pub fn binary_search<T>(is_ok: &dyn Fn(&T) -> bool, arr: &[T]) -> usize {
    let mut lo = 0usize;
    let mut hi = arr.len();
    while lo < hi {
        let i = (lo + hi - 1) >> 1;
        if is_ok(&arr[i]) {
            hi = i;
        } else {
            lo = i + 1;
        }
    }
    hi
}

pub fn lower_bound<T: Ord>(arr: &[T], x: &T) -> usize {
    let is_ok = |y: &T| y >= x;
    binary_search(&is_ok, arr)
}

pub fn upper_bound<T: Ord>(arr: &[T], x: &T) -> usize {
    let is_ok = |y: &T| y > x;
    binary_search(&is_ok, arr)
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_search() {
        let v = (0..10).collect::<Vec<_>>();
        assert_eq!(super::binary_search(&|x| x >= &5, &v), 5);
        assert_eq!(super::binary_search(&|x| x >= &10, &v), 10);
        assert_eq!(super::binary_search(&|x| x >= &11, &v), 10);
        assert_eq!(super::lower_bound(&v, &-1), 0);
        assert_eq!(super::upper_bound(&v, &-1), 0);
        assert_eq!(super::lower_bound(&v, &0), 0);
        assert_eq!(super::upper_bound(&v, &0), 1);
        assert_eq!(super::lower_bound(&v, &15), 10);
        assert_eq!(super::upper_bound(&v, &15), 10);
    }
}
