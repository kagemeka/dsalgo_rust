/// O(1)
pub fn popcount(n: u64) -> u8 { n.count_ones() as u8 }

/// O(\log\log{N})
pub fn popcount_divide_conquer(mut n: u64) -> u8 {
    n -= (n >> 1) & 0x5555555555555555;
    n = (n & 0x3333333333333333) + ((n >> 2) & 0x3333333333333333);
    n = (n + (n >> 4)) & 0x0f0f0f0f0f0f0f0f;
    n = n + (n >> 8);
    n = n + (n >> 16);
    n = n + (n >> 32);
    return (n & 0x0000007f) as u8;
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
