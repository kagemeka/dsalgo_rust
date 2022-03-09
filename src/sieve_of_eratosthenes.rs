pub fn sieve_of_eratosthens(sieve_size: usize) -> Vec<bool> {
    if sieve_size < 2 {
        vec![false; sieve_size];
    }
    let mut is_prime = vec![true; sieve_size];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..sieve_size {
        if i * i >= sieve_size {
            break;
        }
        if !is_prime[i] {
            continue;
        }
        for j in (i * i..sieve_size).step_by(i) {
            is_prime[j] = false;
        }
    }
    is_prime
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let sieve = super::sieve_of_eratosthens(1 << 4);
        assert_eq!(
            sieve.into_iter().take(10).collect::<Vec<_>>(),
            vec![
                false, false, true, true, false, true, false, true, false, false
            ],
        );
    }
}
