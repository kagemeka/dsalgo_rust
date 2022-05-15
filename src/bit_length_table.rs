/// O(N)
pub fn bit_length_table(size: usize) -> Vec<u8> {
    let mut length = vec![0; size as usize];
    for i in 1..size {
        length[i] = length[i >> 1] + 1;
    }
    length
}
