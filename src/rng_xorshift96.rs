pub fn xorshift96(seeds: [u32; 3]) -> [u32; 3] {
    let [mut x, y, z] = seeds;
    x ^= (x << 3) ^ y ^ (y >> 19) ^ z ^ (z << 6);
    [y, z, x]
}

pub struct XorShift96 {
    seeds: [u32; 3],
}

impl XorShift96 {
    pub fn new(seeds: [u32; 3]) -> Self { Self { seeds } }

    pub fn next(&mut self) -> u32 {
        self.seeds = xorshift96(self.seeds);
        self.seeds[2]
    }
}

impl Default for XorShift96 {
    fn default() -> Self { Self::new([123456789, 362436069, 521288629]) }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
