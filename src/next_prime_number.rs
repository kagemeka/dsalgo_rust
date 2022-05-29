// TODO: use miller rabin
pub fn next_prime(mut n: u64) -> u64 {
    loop {
        n += 1;
        if is_prime_naive(n) {
            break;
        }
    }
    n
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
