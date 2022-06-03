pub fn extgcd_recurse(a: i64, b: i64) -> (u64, i64, i64) {
    if b == 0 {
        return if a < 0 { ((-a) as u64, -1, 0) } else { (a as u64, 1, 0) };
    }
    let (g, s, t) = extgcd_recurse(b, a % b);
    (g, t, s - a / b * t)
}

pub fn extgcd(mut a: i64, mut b: i64) -> (u64, i64, i64) {
    let (mut x00, mut x01, mut x10, mut x11) = (1, 0, 0, 1);
    while b != 0 {
        let q = a / b;
        // (x00, x01) = (x01, x00 - q * x01);
        // (x10, x11) = (x11, x10 - q * x11);
        // (a, b) = (b, a % b);
        x00 -= q * x01;
        std::mem::swap(&mut x00, &mut x01);
        x10 -= q * x11;
        std::mem::swap(&mut x10, &mut x11);
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    if a < 0 {
        a = -a;
        x00 = -x00;
        x10 = -x10;
    }
    (a as u64, x00, x10)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            extgcd_recurse(-30, 111),
            (3, 11, 3)
        );
        assert_eq!(extgcd_recurse(0, 0), (0, 1, 0));
        assert_eq!(extgcd(-30, 111), (3, 11, 3));
        assert_eq!(extgcd(111, 30), (3, 3, -11));
        assert_eq!(extgcd(0, 0), (0, 1, 0));
    }
}
