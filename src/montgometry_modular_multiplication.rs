use crate::bit_length::bit_length;

/// reference
/// https://ja.wikipedia.org/wiki/
/// %E3%83%A2%E3%83%B3%E3%82%B4%E3%83%A1%E3%83%AA%E4%B9%97%E7%AE%97
pub struct MontgomeryMultiplication {
    n: u128,
    n_bit_len: u8,
    mask: u128,
    r2: u128,
    n_dash: u128,
}

impl MontgomeryMultiplication {
    pub fn new(modulus: u64) -> Self {
        assert!(modulus & 1 == 1 && modulus >= 3);
        let n = modulus as u128;
        let n_bit_len = bit_length(modulus);
        let r = 1u128 << n_bit_len; // gcd(r, n) = 1, r > n, 2^?
        let r2 = (1u128 << (n_bit_len << 1)) % n;
        let mask = r - 1;
        let mut n_dash = 0; // n*n_dash \equiv -1 \mod r
        let mut t = 0;
        for i in 0..n_bit_len {
            if t & 1 == 0 {
                t += n;
                n_dash |= 1 << i;
            }
            t >>= 1;
        }
        Self { n, n_bit_len, mask, r2, n_dash }
    }

    fn montgomery_reduction(&self, mut t: u128) -> u128 {
        t = (t + ((t & self.mask) * self.n_dash & self.mask) * self.n)
            >> self.n_bit_len;
        if t >= self.n {
            t -= self.n;
        }
        t
    }

    pub fn mul(&self, x: u64, y: u64) -> u64 {
        self.montgomery_reduction(
            self.montgomery_reduction(x as u128 * y as u128) * self.r2,
        ) as u64
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
