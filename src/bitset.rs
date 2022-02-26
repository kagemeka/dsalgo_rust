/// O(1)
pub fn most_significant_bit(n: usize) -> isize {
    0usize.leading_zeros() as isize - n.leading_zeros() as isize - 1
}

/// O(1)
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

pub fn least_significant_bit(n: usize) -> usize { n.reverse_bits().leading_zeros() as usize }

pub fn lsb_number(n: usize) -> usize {
    let n = n as isize;
    (n & -n) as usize
}

pub fn delete_least_bit(n: usize) -> usize { n - lsb_number(n) }

pub fn delete_least_bit_v2(n: usize) -> usize { n & (n - 1) }

/// O(1)
pub fn bit_length(n: usize) -> usize { (0usize.leading_zeros() - n.leading_zeros()) as usize }

/// O(1)
pub fn bit_length_v2(mut n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut l = 1;
    for i in (0..6usize).rev() {
        if n >> (1 << i) > 0 {
            n >>= 1 << i;
            l += 1 << i;
        }
    }
    l
}

/// O(\log{N})
pub fn bit_length_v3(n: usize) -> usize {
    let mut l = 0usize;
    while 1 << l <= n {
        l += 1;
    }
    l
}

/// O(N)
pub fn bit_length_table(n: usize) -> Vec<usize> {
    let mut l = vec![0; n];
    for i in 1..n {
        l[i] = l[i >> 1] + 1;
    }
    l
}

/// O(1)
pub fn reverse_bits(n: usize) -> usize { n.reverse_bits() }

/// O(1)
pub fn reverse_bits_v2(mut n: usize) -> usize {
    n = ((n & 0xaaaaaaaaaaaaaaaa) >> 1) | ((n & 0x5555555555555555) << 1);
    n = ((n & 0xcccccccccccccccc) >> 2) | ((n & 0x3333333333333333) << 2);
    n = ((n & 0xf0f0f0f0f0f0f0f0) >> 4) | ((n & 0x0f0f0f0f0f0f0f0f) << 4);
    n = ((n & 0xff00ff00ff00ff00) >> 8) | ((n & 0x00ff00ff00ff00ff) << 8);
    n = ((n & 0xffff0000ffff0000) >> 8) | ((n & 0x0000ffff0000ffff) << 16);
    (n >> 32) | (n << 32)
}

/// O(1)
pub fn bit_inverse(n: usize) -> usize { !n }

/// O(1)
pub fn bit_inverse_v2(n: usize) -> usize { (!0usize) ^ n }

/// O(1)
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
pub fn popcount_v2(n: usize) -> usize { n.count_ones() as usize }

/// O(\log{N})
pub fn popcount_v3(mut n: usize) -> usize {
    let mut cnt = 0usize;
    while n > 0 {
        cnt += n & 1;
        n >>= 1
    }
    cnt
}

/// O(N)
pub fn popcount_table(n: usize) -> Vec<usize> {
    let mut cnt = vec![0; n];
    for i in 1..n {
        cnt[i] = cnt[i >> 1] + (i & 1);
    }
    cnt
}

#[cfg(test)]
mod tests {
    #[test]
    fn bitset() {
        println!("{}", super::least_significant_bit(0));
    }
}
