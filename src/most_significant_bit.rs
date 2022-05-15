use crate::bit_length::bit_length;

pub fn msb(n: u64) -> Option<usize> {
    let length = bit_length(n) as usize;
    if length == 0 { None } else { Some(length - 1) }
}
