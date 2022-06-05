pub struct Xorwow {
    seeds: [u32; 5],
    counter: u32,
}

impl Xorwow {
    pub fn new(seeds: [u32; 5], counter: u32) -> Self {
        Self { seeds, counter }
    }

    pub fn next(&mut self) -> u32 {
        let [mut x, y, z, w, v] = self.seeds;
        x ^= x >> 2;
        x ^= (x << 1) ^ v ^ (v << 4);
        const DELTA: u32 = 362437;
        if self.counter <= std::u32::MAX - DELTA {
            self.counter += DELTA;
        } else {
            self.counter = DELTA - (std::u32::MAX - self.counter);
        }
        x += self.counter;
        self.seeds = [y, z, w, x, v];
        x
    }
}

impl Default for Xorwow {
    fn default() -> Self {
        Xorwow {
            seeds: [
                123456789, 362436069, 521288629, 88675123, 5783321,
            ],
            counter: 6615241,
        }
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
