pub fn reset_bit(n: u64, bit: usize) -> u64 { n & !(1 << bit) }
