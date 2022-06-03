use crate::greatest_common_divisor::gcd;

/// in the context, we define
/// _{\gcd}|prod{\emptyset} := 0
/// \gcd(0, 0) := 0
pub fn gcd_reduce<I>(iter: I) -> u64
where
    I: Iterator<Item = u64>,
{
    iter.fold(0, |a, b| {
        if a == 0 && b == 0 { 0 } else { gcd(a, b) }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(gcd_reduce([].into_iter()), 0);
        assert_eq!(
            gcd_reduce([2, 8, 4].into_iter()),
            2
        );
    }
}
