use crate::{bit_length::bit_length, karatsuba_quotient::karatsuba_quotient};

pub struct BarrettReduction {
    n: u128,
    k: u8,
    m: u128,
}

impl BarrettReduction {
    pub fn new(modulus: u64) -> Self {
        assert!(0 < modulus);
        let n = modulus as u128;
        let k = bit_length(modulus) << 1;
        let m = (1u128 << k) / n;
        Self { n, k, m }
    }

    // TODO: any faster algorithm than karatsuba to avoid overflow?
    pub fn reduce(&self, mut x: u128) -> u64 {
        assert!(x < self.n.pow(2));
        let q = karatsuba_quotient(self.k >> 1, x, self.m);
        x -= q * self.n;
        if x >= self.n {
            x -= self.n;
        }
        debug_assert!(x < self.n);
        x as u64
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
