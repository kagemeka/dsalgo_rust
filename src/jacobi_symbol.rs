/// compute jacobi symbol (a/modulus)
/// https://en.wikipedia.org/wiki/Jacobi_symbol
pub fn jacobi_symbol(modulus: u64, mut a: u64) -> i8 {
    let mut n = modulus;
    assert!(n > 0 && n & 1 == 1);
    assert!(a < n);
    let mut symbol = 1;
    while a != 0 {
        while a & 1 == 0 {
            a >>= 1;
            let r = n & 7; // n % 8
            if r == 3 || r == 5 {
                symbol = -symbol;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if a & 3 == 3 && n & 3 == 3 {
            symbol = -symbol;
        }
        a %= n;
    }
    if n != 1 { 0 } else { symbol }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
