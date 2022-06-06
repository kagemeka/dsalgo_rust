pub fn prime_factorize_trial_division(mut n: u64) -> Vec<(u64, u8)> {
    assert!(n > 0);
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
    fn test() {
        use super::*;
        assert_eq!(
            prime_factorize_trial_division(9999999999999),
            [
                (3, 2),
                (53, 1),
                (79, 1),
                (265371653, 1)
            ],
        );

        assert_eq!(
            prime_factorize_trial_division(9316358251200),
            [
                (2, 6),
                (3, 3),
                (5, 2),
                (7, 1),
                (11, 1),
                (13, 1),
                (17, 1),
                (19, 1),
                (23, 1),
                (29, 1)
            ],
        );
    }
}
