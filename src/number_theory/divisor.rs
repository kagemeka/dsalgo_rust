pub fn find_divisors(n: i32) -> Vec<i32> {
    let mut divisors: Vec<i32> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i * i >= n { break; }
        if n % i != 0 { continue; }
        divisors.push(i);
        divisors.push(n / i);
    }
    if i * i == n { divisors.push(i); }
    divisors.sort();
    divisors
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_divisors() {
        assert_eq!(find_divisors(25), vec![1, 5, 25]);
    }
    
}