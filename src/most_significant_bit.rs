use crate::bit_length::bit_length;

pub fn msb(n: u64) -> usize {
    assert!(n > 0);
    // it's trivial that msb of 0 is undefined.
    // if n = 0, it's wrong with the caller.
    bit_length(n) as usize - 1
}
