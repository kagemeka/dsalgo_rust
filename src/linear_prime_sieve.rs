/// compute least prime factor table and prime numbers list.
pub fn linear_prime_sieve(size: usize) -> (Vec<Option<u32>>, Vec<u32>) {
    let mut lpf = vec![None; size];
    let mut prime_numbers = Vec::with_capacity(size);
    for i in 2..size {
        if lpf[i].is_none() {
            lpf[i] = Some(i as u32);
            prime_numbers.push(i as u32);
        }
        for &p in &prime_numbers {
            if p > lpf[i].unwrap() || p as usize * i >= size {
                break;
            }
            debug_assert!(lpf[p as usize * i].is_none());
            lpf[p as usize * i] = Some(p);
        }
    }
    (lpf, prime_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        use crate::{
            find_prime_numbers::find_prime_numbers,
            least_prime_factor_table::least_prime_factor_table,
        };
        const K: usize = 1 << 10;
        let lpf_ans = least_prime_factor_table(K);
        let primes_ans = find_prime_numbers(K as u32);
        assert_eq!(
            (lpf_ans, primes_ans),
            linear_prime_sieve(K)
        );
    }
}
