use crate::rng_xorshift64::xorshift64;

pub fn static_xorshift64() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering::SeqCst};
    const DEFAULT_SEED: u64 = 88172645463325252;
    static CELL: AtomicU64 = AtomicU64::new(DEFAULT_SEED);
    let mut x = CELL.load(SeqCst);
    x = xorshift64(x);
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
