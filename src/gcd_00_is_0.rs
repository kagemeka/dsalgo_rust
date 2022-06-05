use crate::greatest_common_divisor::gcd;

pub fn gcd_00_is_0(a: u64, b: u64) -> u64 {
    if a == 0 && b == 0 { 0 } else { gcd(a, b) }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
