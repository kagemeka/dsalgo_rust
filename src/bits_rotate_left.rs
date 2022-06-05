/// can be called safely only in release mode.
pub fn bits_rotate_left(x: u64, k: u8) -> u64 { (x << k) | (x >> (64 - k)) }

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
