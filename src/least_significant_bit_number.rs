use crate::least_significant_bit::least_significant_bit;

/// ```
/// use dsalgo::least_significant_bit_number::lsb_number_u32;
/// assert_eq!(lsb_number_u32(0), 0);
/// assert_eq!(lsb_number_u32(1), 1);
/// assert_eq!(lsb_number_u32(2), 2);
/// assert_eq!(lsb_number_u32(3), 1);
/// ```
pub fn lsb_number_u32(n: u32) -> u32 { ((n as i64) & -(n as i64)) as u32 }

pub fn lsb_number(n: u64) -> u64 {
    match least_significant_bit(n) {
        Some(bit) => 1 << bit,
        None => 0,
    }
}
