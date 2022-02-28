pub fn greatest_common_divisor_recurse(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        greatest_common_divisor_recurse(b, a % b)
    }
}

pub fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

pub fn gcd_prod(slice: &[usize]) -> usize {
    slice
        .iter()
        .fold(0, |accumulated, x| greatest_common_divisor(accumulated, *x))
}

pub fn least_common_multiple(a: usize, b: usize) -> usize {
    if a == 0 && b == 0 {
        0
    } else {
        a / greatest_common_divisor(a, b) * b
    }
}

pub fn extended_euclidean_recurse(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, s, t) = extended_euclidean_recurse(b, a % b);
    (g, t, s - a / b * t)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::greatest_common_divisor_recurse(10, 5), 5);
        assert_eq!(super::greatest_common_divisor_recurse(0, 10), 10);
        assert_eq!(super::greatest_common_divisor_recurse(0, 0), 0);
        assert_eq!(super::greatest_common_divisor(10, 5), 5);
        assert_eq!(super::greatest_common_divisor(0, 10), 10);
        assert_eq!(super::greatest_common_divisor(0, 0), 0);
        assert_eq!(super::gcd_prod(&[]), 0);
        assert_eq!(super::gcd_prod(&[2, 8, 4]), 2);
        assert_eq!(super::least_common_multiple(0, 0), 0);
        assert_eq!(super::least_common_multiple(1, 0), 0);
        assert_eq!(super::least_common_multiple(12, 18), 36);
        assert_eq!(super::extended_euclidean_recurse(-30, 111), (3, 11, 3));
    }
}
