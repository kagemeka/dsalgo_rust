use crate::bit_length::bit_length;

// TODO: role up same codes with 64bit const R version as trait.
/// reference
/// https://ja.wikipedia.org/wiki/
/// %E3%83%A2%E3%83%B3%E3%82%B4%E3%83%A1%E3%83%AA%E4%B9%97%E7%AE%97
pub struct MontgomeryMultiplication {
    n: u128,
    n_bit_len: u8,
    mask: u128,
    r2: u128,
    n_dash: u128,
    nr: u128,
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
        // decide n_dash so that n*n_dash \equiv -1 \equiv r - 1 = mask
        let mut t = 0;
        // t = n*n_dash (manage upper than or equal to i-th bit)
        for i in 0..n_bit_len {
            if t & 1 == 0 {
                // i-th bit is not set.
                t += n;
                // because n is odd, i-th bit is gonna be set.
                n_dash |= 1 << i;
            }
            t >>= 1; // next, check (i+1)-th bit
        }
        debug_assert_eq!(n * n_dash & mask, mask);
        let nr = n * r;
        Self {
            n,
            n_bit_len,
            mask,
            r2,
            n_dash,
            nr,
        }
    }

    /// return tr^{-1} mod n
    fn reduce(&self, mut t: u128) -> u64 {
        assert!(t < self.nr);
        t = (t + ((t & self.mask) * self.n_dash & self.mask) * self.n)
            >> self.n_bit_len;
        if t >= self.n {
            t -= self.n;
        }
        debug_assert!(t < self.n);
        t as u64
    }

    /// return xr mod n
    /// (xr^2)r^{-1} \equiv xr
    #[allow(dead_code)]
    fn form(&self, x: u64) -> u128 { self.reduce(x as u128 * self.r2) as u128 }

    /// return (a * b) mod n
    pub fn mul(&self, x: u64, y: u64) -> u64 {
        self.reduce(self.reduce(x as u128 * y as u128) as u128 * self.r2) as u64
        // ((xyr^{-1})r^2)r^{-1} \equiv xy
        // equivalent to: reduce(form(x) * form(y))
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
