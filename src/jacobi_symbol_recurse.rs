pub fn jacobi_symbol_recurse(n: u64, mut a: u64) -> i8 {
    assert!(a < n && n & 1 == 1);
    if a == 0 {
        return if n != 1 { 0 } else { 1 };
    }
    let mut symbol = 1;
    let ctz = a.trailing_zeros();
    if ctz & 1 == 1 && (n & 7 == 3 || n & 7 == 5) {
        symbol = -symbol;
    }
    a >>= ctz;
    if a & 3 == 3 && n & 3 == 3 {
        symbol = -symbol;
    }
    jacobi_symbol_recurse(a, n % a) * symbol
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::jacobi_symbol::jacobi_symbol;
        for n in (1..100).step_by(2) {
            for a in 0..n {
                assert_eq!(
                    jacobi_symbol_recurse(n, a),
                    jacobi_symbol(n, a),
                );
            }
        }
    }
}
