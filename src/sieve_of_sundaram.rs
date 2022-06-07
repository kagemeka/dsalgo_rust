pub fn sieve_of_sundaram(less_than: u32) -> Vec<u32> {
    let mut prime_numbers = vec![];
    if less_than <= 2 {
        return prime_numbers;
    }
    prime_numbers.push(2);
    let size = (less_than >> 1) as usize;
    let mut is_prime = vec![true; size];
    for i in 1..size {
        if is_prime[i] {
            prime_numbers.push(((i as u32) << 1) | 1);
        }
        for j in (i * (i + 1) << 1..size).step_by((i << 1) | 1) {
            is_prime[j] = false;
        }
    }
    prime_numbers
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::sieve_of_eratosthenes::sieve_of_eratosthenes;
        let limits = [99, 100, 101, 102, 1 << 17];
        for limit in limits {
            assert_eq!(
                sieve_of_sundaram(limit),
                sieve_of_eratosthenes(limit as usize),
            );
        }
    }
}
