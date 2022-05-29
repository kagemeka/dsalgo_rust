use crate::is_prime_table::is_prime_table;

/// table[i] := largest prime number less or equal to i.
pub fn previous_prime_table(size: usize) -> Vec<Option<u64>> {
    let mut prev = is_prime_table(size)
        .into_iter()
        .enumerate()
        .map(
            |(i, is_prime)| {
                if is_prime { Some(i as u64) } else { None }
            },
        )
        .collect::<Vec<_>>();
    for i in 4..size {
        if prev[i].is_none() {
            prev[i] = prev[i - 1];
        }
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let prev = previous_prime_table(20);
        assert_eq!(
            prev.into_iter().take(10).collect::<Vec<_>>(),
            vec![
                None,
                None,
                Some(2),
                Some(3),
                Some(3),
                Some(5),
                Some(5),
                Some(7),
                Some(7),
                Some(7),
            ],
        );
    }
}
