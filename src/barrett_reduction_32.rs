pub struct BarrettReduction32 {
    n: u128,
    m: u128,
}

impl BarrettReduction32 {
    pub fn new(modulus: u32) -> Self {
        let n = modulus as u128;
        let m = (1u128 << 64) / n as u128;
        Self { n, m }
    }

    pub fn reduce(&self, x: u64) -> u32 {
        let mut x = x as u128;
        assert!(x < self.n.pow(2));
        let q = (x as u128 * self.m) >> 64;
        x -= q * self.n;
        if x >= self.n {
            x -= self.n;
        }
        debug_assert!(x < self.n);
        x as u32
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
