use crate::modular_division::divmod;

pub fn floor_div(mut a: i64, mut b: i64) -> i64 {
    if b < 0 {
        a = -a;
        b = -b;
    }
    divmod(a, b).0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(floor_div(10, 3), 3);
        assert_eq!(floor_div(10, -3), -4);
        assert_eq!(floor_div(-10, 3), -4);
        assert_eq!(floor_div(-10, -3), 3);
    }
}
