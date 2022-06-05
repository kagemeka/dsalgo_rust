pub fn xorshift32(seed: u32) -> u32 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}

pub struct XorShift32(u32);

impl XorShift32 {
    pub fn next(&mut self) -> u32 {
        self.0 = xorshift32(self.0);
        self.0
    }
}

impl Default for XorShift32 {
    fn default() -> Self { XorShift32(2463534242) }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
