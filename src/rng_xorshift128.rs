pub fn xorshift128(seeds: [u32; 4]) -> [u32; 4] {
    let [mut x, y, z, w] = seeds;
    x ^= x << 11;
    x ^= (x >> 8) ^ w ^ (w >> 19);
    [y, z, w, x]
}

pub struct XorShift128 {
    seeds: [u32; 4],
}

impl XorShift128 {
    pub fn new(seeds: [u32; 4]) -> Self { Self { seeds } }

    pub fn next(&mut self) -> u32 {
        self.seeds = xorshift128(self.seeds);
        self.seeds[3]
    }
}

impl Default for XorShift128 {
    fn default() -> Self {
        Self::new([
            123456789, 362436069, 521288629, 88675123,
        ])
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
