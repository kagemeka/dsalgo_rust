/// if >= v1.60.0, a.abs_diff(b) is available.
/// https://doc.rust-lang.org/std/primitive.u64.html#method.abs_diff
pub fn abs_diff(a: u64, b: u64) -> u64 { if a >= b { a - b } else { b - a } }

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
