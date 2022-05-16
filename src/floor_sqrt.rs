pub fn floor_sqrt(n: u64) -> u64 {
    let mut lo = 0;
    let mut hi = 1 << 32;
    while hi - lo > 1 {
        let x = (lo + hi) / 2;
        if n / x >= x {
            lo = x;
        } else {
            hi = x;
        }
    }
    lo
}
