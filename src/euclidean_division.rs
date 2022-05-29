pub fn divrem_euclid(a: i64, b: i64) -> (i64, i64) {
    (
        a.div_euclid(b),
        a.rem_euclid(b),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(divrem_euclid(10, 3), (3, 1));
        assert_eq!(divrem_euclid(10, -3), (-3, 1));
        assert_eq!(divrem_euclid(-10, 3), (-4, 2));
        assert_eq!(divrem_euclid(-10, -3), (4, 2));
    }
}
