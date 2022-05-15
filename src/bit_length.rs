/// O(1)
pub fn bit_length(n: u64) -> u8 {
    (0u64.leading_zeros() - n.leading_zeros()) as u8
}

/// O(\log\log{N}})
pub fn bit_length_binary_search(mut n: u64) -> u8 {
    let mut length = 0;
    for i in (0..6).rev() {
        let l = 1 << i;
        if n >> l > 0 {
            n >>= l;
            length += l;
        }
    }
    if n == 1 {
        length += 1;
        n -= 1;
    }
    assert_eq!(n, 0);
    length
}

/// O(\log{N})
pub fn bit_length_naive(mut n: u64) -> u8 {
    let mut length = 0;
    while n > 0 {
        n >>= 1;
        length += 1;
    }
    length
}
