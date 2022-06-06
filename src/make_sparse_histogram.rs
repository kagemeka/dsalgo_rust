pub fn make_sparse_histogram<T>(mut values: Vec<T>) -> Vec<(T, u64)>
where
    T: Ord + Clone,
{
    values.sort();
    let mut iter = values.into_iter();
    let mut res = vec![];
    let mut value: T;
    let mut count: u64;
    if let Some(v) = iter.next() {
        value = v;
        count = 1;
    } else {
        return res;
    }
    for v in iter {
        if v == value {
            count += 1;
        } else {
            res.push((value, count));
            value = v;
            count = 1;
        }
    }
    res.push((value, count));
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(
            make_sparse_histogram(vec![1, 1, 1, 2, 2, 2, 3, 3, 3]),
            vec![(1, 3), (2, 3), (3, 3)]
        );
    }
}
