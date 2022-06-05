use crate::rng_xoshiro256_core::xorshiro256_core;

pub struct Xoshiro256Plus {
    seeds: [u64; 4],
}

impl Xoshiro256Plus {
    pub fn new(seeds: [u64; 4]) -> Self { Self { seeds } }

    pub fn next(&mut self) -> u64 {
        let res = self.seeds[0] + self.seeds[3];
        self.seeds = xorshiro256_core(self.seeds);
        res
    }
}

impl Default for Xoshiro256Plus {
    fn default() -> Self { Self::new([1; 4]) }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
