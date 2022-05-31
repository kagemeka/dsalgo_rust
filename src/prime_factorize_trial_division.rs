pub fn prime_factorize_trial_division(mut n: u64) -> Vec<(u64, u8)> {
    let mut factors = vec![];
    for i in 2..n {
        if i * i > n {
            break;
        }
        if n % i != 0 {
            continue;
        }
        let mut cnt = 0;
        while n % i == 0 {
            cnt += 1;
            n /= i;
        }
        factors.push((i, cnt));
    }
    if n != 1 {
        factors.push((n, 1));
    }
    factors
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
