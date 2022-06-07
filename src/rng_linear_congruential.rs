pub struct RngLinearCongruentialGenerator {
    m: u128,
    a: u128,
    b: u128,
}

impl RngLinearCongruentialGenerator {
    pub fn new(modulus: u64, a: u64, b: u64) -> Self {
        assert!(0 < a && a < modulus && b < modulus);
        Self {
            m: modulus as u128,
            a: a as u128,
            b: b as u128,
        }
    }

    pub fn next(&mut self, x: u64) -> u64 {
        ((self.a * x as u128 + self.b) % self.m) as u64
    }
}

pub struct RngLCGCell {
    lcg: RngLinearCongruentialGenerator,
    x: u64,
}

impl RngLCGCell {
    pub fn new(lcg: RngLinearCongruentialGenerator, x0: u64) -> Self {
        Self { lcg, x: x0 }
    }

    pub fn next(&mut self) -> u64 {
        self.x = self.lcg.next(self.x);
        self.x
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
