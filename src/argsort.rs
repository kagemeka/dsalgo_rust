pub fn argsort<T: std::cmp::Ord>(a: &Vec<T>) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..a.len()).collect();
    idx.sort_by_key(|&i| &a[i]);
    idx
}
