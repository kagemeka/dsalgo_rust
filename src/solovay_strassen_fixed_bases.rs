use crate::is_composite_euler_jacobi::is_composite_euler_jacobi;

pub struct SolovayStrassenFixedBases {
    bases: Vec<u64>,
}

impl SolovayStrassenFixedBases {
    pub fn new(bases: Vec<u64>) -> Self { Self { bases } }

    // TODO: implement as common trait.
    pub fn from_random_bases(epochs: u8) -> Self {
        use crate::rng_static_xorshift64::static_xorshift64;
        Self::new((0..epochs).map(|_| static_xorshift64()).collect::<Vec<_>>())
    }

    pub fn is_prime(&self, n: u64) -> bool {
        if n == 2 {
            return true;
        }
        if n < 2 || n & 1 == 0 {
            return false;
        }
        // [2, n)
        self.bases
            .iter()
            .map(|&base| base % n)
            .filter(|&base| 2 <= base && base < n)
            .all(|b| !is_composite_euler_jacobi(b, n))
        // strong probable prime.
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
