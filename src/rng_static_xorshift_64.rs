pub fn static_xorshift_64() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering::SeqCst};
    const DEFAULT_SEED: u64 = 88172645463325252;
    static CELL: AtomicU64 = AtomicU64::new(DEFAULT_SEED);
    let mut x = CELL.load(SeqCst);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    CELL.store(x, SeqCst);
    x
}

// TODO: diehard test
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
    }
}
