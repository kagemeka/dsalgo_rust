/// O(1)
pub fn most_significant_bit(n: usize) -> Option<u32> {
    if n == 0 {
        None
    } else {
        Some(0usize.leading_zeros() - n.leading_zeros() - 1)
    }
}

/// O(\log\log{N})
/// ```
/// use dsalgo::bitwise::msb_number_binary_search;
/// assert_eq!(msb_number_binary_search(0), 0);
/// ```
pub fn msb_number_binary_search(mut n: usize) -> usize {
    if n & 0xffffffff00000000 > 0 {
        n &= 0xffffffff00000000;
    }
    if n & 0xffff0000ffff0000 > 0 {
        n &= 0xffff0000ffff0000;
    }
    if n & 0xff00ff00ff00ff00 > 0 {
        n &= 0xff00ff00ff00ff00;
    }
    if n & 0xf0f0f0f0f0f0f0f0 > 0 {
        n &= 0xf0f0f0f0f0f0f0f0;
    }
    if n & 0xcccccccccccccccc > 0 {
        n &= 0xcccccccccccccccc;
    }
    if n & 0xaaaaaaaaaaaaaaaa > 0 {
        n &= 0xaaaaaaaaaaaaaaaa;
    }
    n
}

pub fn least_significant_bit(n: usize) -> Option<u32> {
    if n == 0 {
        None
    } else {
        Some(n.reverse_bits().leading_zeros())
    }
}

pub fn lsb_number_direct(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        ((n as isize) & -(n as isize)) as usize
    }
}

/// ```
/// use dsalgo::bitwise::lsb_number;
/// assert_eq!(lsb_number(0), 0);
/// assert_eq!(lsb_number(1), 1);
/// assert_eq!(lsb_number(2), 2);
/// assert_eq!(lsb_number(3), 1);
/// ```
pub fn lsb_number(n: usize) -> usize { n - reset_least_bit(n) }

pub fn reset_least_bit_naive(n: usize) -> usize { n - lsb_number_direct(n) }

/// ```
/// use dsalgo::bitwise::reset_least_bit;
/// assert_eq!(reset_least_bit(0), 0);
/// assert_eq!(reset_least_bit(16), 0);
/// assert_eq!(reset_least_bit(3), 2);
/// ```
pub fn reset_least_bit(n: usize) -> usize { if n == 0 { 0 } else { n & (n - 1) } }

pub fn reset_bit(n: usize, bit: usize) -> usize { n & !(1 << bit) }

/// O(\log{N})
pub fn bit_length_naive(n: usize) -> u32 {
    let mut length = 0;
    while 1 << length <= n {
        length += 1;
    }
    length
}

/// O(\log\log{N}})
pub fn bit_length_binary_search(mut n: usize) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut length = 1;
    for i in (0..6usize).rev() {
        if n >> (1 << i) > 0 {
            n >>= 1 << i;
            length += 1 << i;
        }
    }
    length
}

/// O(1)
pub fn bit_length(n: usize) -> u32 { 0usize.leading_zeros() - n.leading_zeros() }

/// O(N)
pub fn bit_length_table(n: usize) -> Vec<usize> {
    let mut length = vec![0; n];
    for i in 1..n {
        length[i] = length[i >> 1] + 1;
    }
    length
}

/// O(1)
pub fn reverse_bits(n: usize) -> usize { n.reverse_bits() }

/// O(\log\log{N}})
pub fn reverse_bits_butterfly(mut n: usize) -> usize {
    n = ((n & 0xaaaaaaaaaaaaaaaa) >> 1) | ((n & 0x5555555555555555) << 1);
    n = ((n & 0xcccccccccccccccc) >> 2) | ((n & 0x3333333333333333) << 2);
    n = ((n & 0xf0f0f0f0f0f0f0f0) >> 4) | ((n & 0x0f0f0f0f0f0f0f0f) << 4);
    n = ((n & 0xff00ff00ff00ff00) >> 8) | ((n & 0x00ff00ff00ff00ff) << 8);
    n = ((n & 0xffff0000ffff0000) >> 8) | ((n & 0x0000ffff0000ffff) << 16);
    (n >> 32) | (n << 32)
}

/// O(1)
pub fn invert_bits_built_in(n: usize) -> usize { !n }

/// O(1)
pub fn invert_bits(n: usize) -> usize { (!0usize) ^ n }

/// O(\log\log{N})
pub fn popcount(mut n: usize) -> usize {
    n -= (n >> 1) & 0x5555555555555555;
    n = (n & 0x3333333333333333) + ((n >> 2) & 0x3333333333333333);
    n = (n + (n >> 4)) & 0x0f0f0f0f0f0f0f0f;
    n = n + (n >> 8);
    n = n + (n >> 16);
    n = n + (n >> 32);
    return n & 0x0000007f;
}

/// O(1)
pub fn popcount_built_in(n: usize) -> u32 { n.count_ones() }

/// O(\log{N})
pub fn popcount_naive(mut n: usize) -> u32 {
    let mut count = 0;
    while n > 0 {
        count += (n & 1) as u32;
        n >>= 1
    }
    count
}

/// O(N)
pub fn popcount_table(n: usize) -> Vec<usize> {
    let mut count = vec![0; n];
    for i in 1..n {
        count[i] = count[i >> 1] + (i & 1);
    }
    count
}

/// ```
/// use dsalgo::bitwise::shift_right_until_odd;
/// assert_eq!(shift_right_until_odd(0), None);
/// assert_eq!(shift_right_until_odd(1), Some(1));
/// assert_eq!(shift_right_until_odd(2), Some(1));
/// assert_eq!(shift_right_until_odd(12), Some(3));
/// ```
pub fn shift_right_until_odd(n: usize) -> Option<usize> {
    if n == 0 {
        None
    } else {
        Some(n / lsb_number(n))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bitwise() {
        assert_eq!(super::least_significant_bit(0), None);
        assert_eq!(super::least_significant_bit(1), Some(0));
    }
}
