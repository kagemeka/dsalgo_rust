pub fn sieve_of_sundaram(less_than: u64) -> Vec<u64> {
    let size = (less_than >> 1) as usize;
    let mut is_prime = vec![true; size];
    for i in 1..size {
        if 2 * i * (i + 1) >= size {
            break;
        }
        for j in i..size {
            let j = i + j + 2 * i * j;
            if j >= size {
                break;
            }
            is_prime[j] = false;
        }
    }
    let mut prime_numbers = vec![];
    if less_than > 2 {
        prime_numbers.push(2);
    }
    for i in 1..size {
        if is_prime[i] {
            prime_numbers.push((2 * i + 1) as u64);
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
        assert_eq!(
            sieve_of_sundaram(100000),
            sieve_of_eratosthenes(100000),
        );
    }
}
