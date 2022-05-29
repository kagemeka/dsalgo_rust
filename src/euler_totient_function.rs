//! related
//! - charmichael_function.rs

// TODO:
// pass reference of factorization funtion (or closure) as parameter.
pub fn euler_totient(mut n: usize) -> usize {
    let mut cnt = n;
    let mut i = 1;
    while i * i < n {
        i += 1;
        if n % i != 0 {
            continue;
        }
        cnt = cnt / i * (i - 1);
        while n % i == 0 {
            n /= i;
        }
    }
    if n > 1 {
        cnt = cnt / n * (n - 1);
    }
    cnt
}

// TODO: deprecated
/// O(N\log\log{N} + O(\log{N}))
pub struct EulerTotientLPF {}

// TODO:
#[cfg(test)]
mod tests {
    // TODO:
    // test with lpf
    #[test]
    fn test() {}
}
