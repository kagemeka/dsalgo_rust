pub fn find_divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    for divisor in 1..=n {
        if divisor * divisor > n {
            break;
        }
        if n % divisor != 0 {
            continue;
        }
        divisors.push(divisor);
        if divisor * divisor != n {
            divisors.push(n / divisor);
        }
    }
    divisors.sort();
    divisors
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_divisors() {
        assert_eq!(super::find_divisors(25), vec![1, 5, 25]);
    }
}
