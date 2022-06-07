use crate::find_prime_numbers::find_prime_numbers;

pub fn is_prime_table(size: usize) -> Vec<bool> {
    let mut is_prime = vec![false; size];
    for p in find_prime_numbers(size as u32) {
        is_prime[p as usize] = true;
    }
    is_prime
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let is_prime = is_prime_table(20);
        assert_eq!(
            is_prime,
            vec![
                false, false, true, true, false, true, false, true, false,
                false, false, true, false, true, false, false, false, true,
                false, true
            ],
        );
    }
}
