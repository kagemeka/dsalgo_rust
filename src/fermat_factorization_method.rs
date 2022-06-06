use crate::int_sqrt_binary_search::int_sqrt_binary_search;

/// for odd integer n
/// n = ab (\exist a, b are odd)
/// a = x + y
/// b = x - y
/// n = x^2 - y^2
/// x^2 = n + y^2
/// brute force y=0..n/2 to find x. (because x > y)
/// return x + y
pub fn fermat_factorization_method(n: u64) -> u64 {
    assert!(n & 1 == 1);
    let mut x2 = n;
    for y in 0..=n / 2 {
        let x = int_sqrt_binary_search(x2);
        if x * x == x2 {
            debug_assert!((x + y) * (x - y) == n);
            return x + y;
        }
        x2 += y << 1 | 1;
    }
    panic!("can't be here");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
