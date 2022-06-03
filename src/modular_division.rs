pub fn divmod(a: i64, b: i64) -> (i64, i64) {
    assert!(b != 0);
    let (mut q, mut r) = (a / b, a % b);
    if b.signum() * r.signum() == -1 {
        q -= 1;
        r += b;
    }
    (q, r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(divmod(10, 3), (3, 1));
        assert_eq!(divmod(10, -3), (-4, -2));
        assert_eq!(divmod(-10, 3), (-4, 2));
        assert_eq!(divmod(-10, -3), (3, -1));
    }
}
