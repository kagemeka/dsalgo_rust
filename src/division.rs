pub fn divmod_isize(a: isize, b: isize) -> (isize, isize) {
    assert!(b != 0);
    let (mut q, mut r) = (a / b, a % b);
    if b > 0 && r < 0 || b < 0 && r > 0 {
        q -= 1;
        r += b;
    }
    (q, r)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::divmod_isize(10, 3), (3, 1));
        assert_eq!(super::divmod_isize(10, -3), (-4, -2));
        assert_eq!(super::divmod_isize(-10, 3), (-4, 2));
        assert_eq!(super::divmod_isize(-10, -3), (3, -1));
    }
}
