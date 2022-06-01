use crate::modular_power::modular_pow_64;

pub(crate) fn is_precise_composite(base: u64, n: u64) -> bool {
    debug_assert!(n > 2 && n & 1 == 1);
    let (mut s, mut d) = (0, n - 1);
    // n - 1 = 2^s*d
    while d & 1 == 0 {
        s += 1;
        d >>= 1;
    }
    let mut x = modular_pow_64(n, base as u128, d) as u128;
    if x == 1 {
        return false;
    }
    let n = n as u128;
    for _ in 0..s {
        if x == n - 1 {
            return false;
        }
        x = x * x % n;
    }
    true
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
