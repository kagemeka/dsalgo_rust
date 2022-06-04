use crate::{
    miller_rabin_deterministic_bases::MILLER_RABIN_BASES_64_FEW,
    miller_rabin_is_definite_composite::is_composite_miller_rabin,
};

pub struct MillerRabinFixedBases {
    bases: Vec<u64>,
}

impl Default for MillerRabinFixedBases {
    fn default() -> Self { Self::new(MILLER_RABIN_BASES_64_FEW.to_vec()) }
}

impl MillerRabinFixedBases {
    pub fn new(bases: Vec<u64>) -> Self { Self { bases } }

    // TODO: impl as common trait
    pub fn from_random_bases(epochs: u8) -> Self {
        use crate::rng_static_xorshift_64::static_xorshift_64;
        Self::new((0..epochs).map(|_| static_xorshift_64()).collect::<Vec<_>>())
    }

    pub fn is_prime(&self, n: u64) -> bool {
        if n == 2 {
            return true;
        }
        if n < 2 || n & 1 == 0 {
            return false;
        }
        // [2, n - 1)
        self.bases
            .iter()
            .map(|&base| base % n)
            .filter(|&base| 2 <= base && base < n - 1)
            .all(|b| !is_composite_miller_rabin(b, n))
        // strong probable prime.
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
