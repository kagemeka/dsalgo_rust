/// O(\sqrt{N})
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

/// O(N\log\log{N} + O(\log{N}))
pub struct EulerTotientLPF {}
