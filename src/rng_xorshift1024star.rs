pub struct Xorshift1024Star {
    seeds: [u64; 16],
    index: usize,
}

impl Xorshift1024Star {
    pub fn new(seeds: [u64; 16]) -> Self { Self { seeds, index: 0 } }

    pub fn next(&mut self) -> u64 {
        let x = self.seeds[self.index];
        self.index = (self.index + 1) & 15;
        let mut y = self.seeds[self.index];
        y ^= y << 31;
        y ^= y >> 11;
        y ^= x ^ (x >> 30);
        self.seeds[self.index] = y;
        y.wrapping_mul(1181783497276652981u64)
    }
}

impl Default for Xorshift1024Star {
    fn default() -> Self {
        Self::new([
            0x84242f96eca9c41d,
            0xa3c65b8776f96855,
            0x5b34a39f070b5837,
            0x4489affce4f31a1e,
            0x2ffeeb0a48316f40,
            0xdc2d9891fe68c022,
            0x3659132bb12fea70,
            0xaac17d8efa43cab8,
            0xc4cb815590989b13,
            0x5ee975283d71c93b,
            0x691548c86c1bd540,
            0x7910c41d10a1e6a5,
            0x0b5fc64563b3e2a8,
            0x047f7684e9fc949d,
            0xb99181f2d8f685ca,
            0x284600e3f30e38c3,
        ])
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
