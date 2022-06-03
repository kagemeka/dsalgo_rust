/// compile time optimized version of MontgomeryMultiplication.
/// two times faster.
pub struct MontgomeryMultiplication64 {
    n: u128,
    r2: u128,
    n_dash: u128,
    nr: u128,
}

impl MontgomeryMultiplication64 {
    const MASK: u128 = std::u64::MAX as u128;
    const R: u128 = 1 << 64;

    pub fn new(modulus: u64) -> Self {
        assert!(modulus & 1 == 1 && modulus >= 3);
        let n = modulus as u128;
        let r2 = (Self::R % n).pow(2) % n;
        let mut n_dash = 0;
        let mut t = 0;
        for i in 0..64 {
            if t & 1 == 0 {
                t += n;
                n_dash |= 1 << i;
            }
            t >>= 1;
        }
        debug_assert_eq!(
            n * n_dash & Self::MASK,
            Self::MASK
        );
        let nr = n * Self::R;
        Self { n, r2, n_dash, nr }
    }

    fn reduce(&self, mut t: u128) -> u64 {
        assert!(t < self.nr);
        t = (t + ((t & Self::MASK) * self.n_dash & Self::MASK) * self.n) >> 64;
        if t >= self.n {
            t -= self.n;
        }
        debug_assert!(t < self.n);
        t as u64
    }

    #[allow(dead_code)]
    fn form(&self, x: u64) -> u128 { self.reduce(x as u128 * self.r2) as u128 }

    pub fn mul(&self, x: u64, y: u64) -> u64 {
        self.reduce(self.reduce(x as u128 * y as u128) as u128 * self.r2) as u64
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
