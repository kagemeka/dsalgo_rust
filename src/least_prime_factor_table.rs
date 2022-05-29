use crate::sieve_of_eratosthenes::sieve_of_eratosthenes;

pub fn least_prime_factor_table(size: usize) -> Vec<Option<u64>> {
    let mut lpf = sieve_of_eratosthenes(size)
        .into_iter()
        .enumerate()
        .map(
            |(i, is_prime)| {
                if is_prime { Some(i as u64) } else { None }
            },
        )
        .collect::<Vec<_>>();
    for i in 2..size {
        if i * i >= size {
            break;
        }
        debug_assert!(lpf[i].is_some());
        if lpf[i] != Some(i as u64) {
            continue;
        }
        for j in (i * i..size).step_by(i as usize) {
            if let Some(x) = lpf[j] {
                debug_assert!(x < i as u64);
            } else {
                lpf[j] = Some(i as u64);
            }
        }
    }
    lpf
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let lpf = least_prime_factor_table(1 << 10);
        assert_eq!(
            lpf.into_iter().take(10).collect::<Vec<_>>(),
            vec![
                None,
                None,
                Some(2),
                Some(3),
                Some(2),
                Some(5),
                Some(2),
                Some(7),
                Some(2),
                Some(3)
            ],
        );
    }
}
