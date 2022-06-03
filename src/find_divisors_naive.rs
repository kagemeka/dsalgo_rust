pub fn find_divisors_naive(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    for d in 1..=n {
        if d * d > n {
            break;
        }
        if n % d != 0 {
            continue;
        }
        divisors.push(d);
        if d * d != n {
            divisors.push(n / d);
        }
    }
    divisors.sort();
    divisors
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_divisors() {
        assert_eq!(
            find_divisors_naive(25),
            vec![1, 5, 25]
        );
    }
}
