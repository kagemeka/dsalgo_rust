use crate::is_prime_table::is_prime_table;

pub fn next_prime_table(size: usize) -> Vec<u64> {
    const MAX_PRIME_GAP_64: u64 = 1550;
    let mut next = is_prime_table(size + MAX_PRIME_GAP_64 as usize)
        .into_iter()
        .enumerate()
        .map(
            |(i, is_prime)| {
                if is_prime { Some(i as u64) } else { None }
            },
        )
        .collect::<Vec<_>>();
    for i in (1..size + 1550).rev() {
        if next[i - 1].is_none() {
            next[i - 1] = next[i];
        }
    }
    next.into_iter().take(size).map(|x| x.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let next_prime = next_prime_table(20);
        assert_eq!(
            next_prime,
            vec![
                2, 2, 2, 3, 5, 5, 7, 7, 11, 11, 11, 11, 13, 13, 17, 17, 17, 17,
                19, 19,
            ],
        )
    }
}
