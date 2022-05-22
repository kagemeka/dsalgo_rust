use crate::least_significant_bit::lsb;

pub fn lsb_number_u32(n: u32) -> u32 { ((n as i64) & -(n as i64)) as u32 }

pub fn lsb_number(n: u64) -> u64 {
    match n {
        0 => 0,
        n => 1 << lsb(n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(lsb_number_u32(0), 0);
        assert_eq!(lsb_number_u32(1), 1);
        assert_eq!(lsb_number_u32(2), 2);
        assert_eq!(lsb_number_u32(3), 1);

        assert_eq!(lsb_number(0), 0);
        assert_eq!(lsb_number(1), 1);
        assert_eq!(lsb_number(2), 2);
        assert_eq!(lsb_number(3), 1);
    }
}
