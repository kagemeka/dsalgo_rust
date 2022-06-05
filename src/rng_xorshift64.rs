pub fn xorshift64(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

pub fn xorshift64_fast(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 7;
    x ^ (x >> 9)
}

pub struct Xorshift64(u64);

impl Xorshift64 {
    pub fn next(&mut self) -> u64 {
        self.0 = xorshift64(self.0);
        self.0
    }
}

impl Default for Xorshift64 {
    fn default() -> Self { Xorshift64(88172645463325252) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
