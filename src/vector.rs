pub fn unique<T: Ord + Clone>(a: &Vec<T>) -> Vec<T> {
    let mut v = a.to_vec();
    v.sort();
    v.dedup();
    v
}


pub fn compress_array<T: Ord + Clone>(a: &Vec<T>) -> (Vec<usize>, Vec<T>) {
    let v = unique(a);
    let a = a.iter().map(|x| v.binary_search(x).unwrap()).collect::<Vec<_>>();
    (a, v)
} 
