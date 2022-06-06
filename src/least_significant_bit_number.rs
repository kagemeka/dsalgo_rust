use crate::least_significant_bit::lsb;

pub fn lsb_number_i64(n: i64) -> i64 { n & -n }

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
        assert_eq!(lsb_number_i64(0), 0);
        assert_eq!(lsb_number_i64(1), 1);
        assert_eq!(lsb_number_i64(2), 2);
        assert_eq!(lsb_number_i64(3), 1);

        assert_eq!(lsb_number(0), 0);
        assert_eq!(lsb_number(1), 1);
        assert_eq!(lsb_number(2), 2);
        assert_eq!(lsb_number(3), 1);
    }
}
