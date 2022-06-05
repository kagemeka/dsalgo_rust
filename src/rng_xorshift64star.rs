pub struct Xorshift64Star(u64);

impl Xorshift64Star {
    pub fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x
    }
}

impl Default for Xorshift64Star {
    fn default() -> Self { Xorshift64Star(1) }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
