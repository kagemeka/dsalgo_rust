/// ```
/// use dsalgo::reset_least_bit::reset_least_bit;
/// assert_eq!(reset_least_bit(0), 0);
/// assert_eq!(reset_least_bit(16), 0);
/// assert_eq!(reset_least_bit(3), 2);
/// ```
pub fn reset_least_bit(n: u64) -> u64 { if n == 0 { 0 } else { n & (n - 1) } }
