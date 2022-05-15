/// O(1)
pub fn popcount(n: u64) -> u8 { n.count_ones() as u8 }

/// O(\log\log{N})
pub fn popcount_divide_conquer(mut n: u64) -> u8 {
    n -= (n >> 1) & 0x5555555555555555;
    // equivalent to
    // n = (n & 0x5555555555555555) + ((n >> 1) & 0x5555555555555555);
    // note:
    // 0x5 = 0b0101
    // 0x(5(base 16) * 16) = 0b(0101(base 2) * 16)
    n = (n & 0x3333333333333333) + ((n >> 2) & 0x3333333333333333);
    // 0x3 = 0b0011
    n = (n + (n >> 4)) & 0x0f0f0f0f0f0f0f0f;
    // equivalent to
    // n = (n & 0x0f0f0f0f0f0f0f0f) + ((n >> 4) & 0x0f0f0f0f0f0f0f0f);
    // because 4*2 = 8 < 16 = 2^4
    // 0x0f = 0b00001111
    n = n + (n >> 8);
    // because 2^8 = 256 >= 64(bit), masking at this point is needless.
    // instead, we must mask the first seven bits (bit_len(64) is 7)
    n = n + (n >> 16);
    n = n + (n >> 32);
    return (n & 0x7f) as u8;
}

/// O(\log{N})
pub fn popcount_naive(mut n: u64) -> u8 {
    let mut count = 0;
    while n > 0 {
        count += (n & 1) as u8;
        n >>= 1
    }
    count
}
