pub struct Xorshift128Plus {
    seeds: [u64; 2],
}

impl Xorshift128Plus {
    pub fn new(seeds: [u64; 2]) -> Self { Self { seeds } }

    pub fn next(&mut self) -> u64 {
        let [mut x, y] = self.seeds;
        x ^= x << 23;
        x ^= x >> 18;
        x ^= y ^ (y >> 5);
        self.seeds = [y, x];
        x + y
    }
}

impl Default for Xorshift128Plus {
    fn default() -> Self { Self::new([1, 1]) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
