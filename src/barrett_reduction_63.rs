pub struct BarrettReduction63 {
    n: u128,
    m0: u128,
    m1: u128,
}

impl BarrettReduction63 {
    const MASK: u128 = (1u128 << 63) - 1;

    pub fn new(modulus: u64) -> Self {
        let n = modulus as u128;
        assert!(n >> 63 == 0);
        let m = (1u128 << 126) / n;
        let (m1, m0) = (m >> 63, m & Self::MASK);
        Self { n, m0, m1 }
    }

    pub fn reduce(&self, mut x: u128) -> u64 {
        assert!(x < self.n.pow(2));
        let (x1, x0) = (x >> 63, x & Self::MASK);
        let t2 = x1 * self.m1;
        let t0 = x0 * self.m0;
        let t1 = x1 * self.m0 + x0 * self.m1;
        let q = t2 + ((t1 + (t0 >> 63)) >> 63);
        x -= q * self.n;
        if x >= self.n {
            x -= self.n;
        }
        debug_assert!(x < self.n);
        x as u64
    }
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
