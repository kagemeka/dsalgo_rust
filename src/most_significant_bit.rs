use crate::bit_length::bit_length;

pub fn msb(n: u64) -> Option<u8> {
    let length = bit_length(n);
    if length == 0 { None } else { Some(length - 1) }
}
