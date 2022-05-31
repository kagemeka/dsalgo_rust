pub fn legendre_function(mut n: u64, p: u64) -> u64 {
    let mut cnt = 0;
    while n > 0 {
        cnt += n / p;
        n /= p;
    }
    cnt
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
