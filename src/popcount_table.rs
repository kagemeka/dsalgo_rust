/// O(N)
pub fn popcount_table(n: usize) -> Vec<u8> {
    let mut count = vec![0; n];
    for i in 1..n {
        count[i] = count[i >> 1] + (i & 1) as u8;
    }
    count
}
