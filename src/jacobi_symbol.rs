/// compute jacobi symbol (a/modulus)
/// https://en.wikipedia.org/wiki/Jacobi_symbol
pub fn jacobi_symbol(modulus: u64, mut a: u64) -> i8 {
    let mut n = modulus;
    assert!(a < n && n & 1 == 1);
    let mut symbol = 1;
    while a != 0 {
        let ctz = a.trailing_zeros();
        if ctz & 1 == 1 && (n & 7 == 3 || n & 7 == 5) {
            symbol = -symbol;
        }
        a >>= ctz;
        if a & 3 == 3 && n & 3 == 3 {
            symbol = -symbol;
        }
        n %= a;
        std::mem::swap(&mut a, &mut n);
    }
    if n != 1 { 0 } else { symbol }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
