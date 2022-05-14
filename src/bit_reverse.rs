/// O(1)
pub fn reverse_bits(n: u64) -> u64 { n.reverse_bits() }

/// O(\log\log{N}})
pub fn reverse_bits_butterfly(mut n: u64) -> u64 {
    n = ((n & 0xaaaaaaaaaaaaaaaa) >> 1) | ((n & 0x5555555555555555) << 1);
    n = ((n & 0xcccccccccccccccc) >> 2) | ((n & 0x3333333333333333) << 2);
    n = ((n & 0xf0f0f0f0f0f0f0f0) >> 4) | ((n & 0x0f0f0f0f0f0f0f0f) << 4);
    n = ((n & 0xff00ff00ff00ff00) >> 8) | ((n & 0x00ff00ff00ff00ff) << 8);
    n = ((n & 0xffff0000ffff0000) >> 8) | ((n & 0x0000ffff0000ffff) << 16);
    (n >> 32) | (n << 32)
}
