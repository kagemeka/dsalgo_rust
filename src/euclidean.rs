pub fn greatest_common_divisor_recurse(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        greatest_common_divisor_recurse(b, a % b)
    }
}

pub fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
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

pub fn extended_euclidean_recurse(a: isize, b: isize) -> (usize, isize, isize) {
    if b == 0 {
        return if a < 0 {
            ((-a) as usize, -1, 0)
        } else {
            (a as usize, 1, 0)
        };
    }
    let (g, s, t) = extended_euclidean_recurse(b, a % b);
    (g, t, s - a / b * t)
}

pub fn extended_euclidean(mut a: isize, mut b: isize) -> (usize, isize, isize) {
    let (mut x00, mut x01, mut x10, mut x11) = (1, 0, 0, 1);
    while b != 0 {
        let (q, r) = (a / b, a % b);
        (x00, x01) = (x01, x00 - q * x01);
        (x10, x11) = (x11, x10 - q * x11);
        (a, b) = (b, r);
    }
    if a < 0 {
        a *= -1;
        x00 *= -1;
        x10 *= -1;
    }
    (a as usize, x00, x10)
}

/// compute g := \gcd(modulus, n),
/// and modular inverse of n/g in Z[modulus/g].
pub fn extended_euclidean_gcd_modular_inverse(modulus: usize, n: usize) -> (usize, Option<usize>) {
    assert!(modulus > 1 && n < modulus);
    if n == 0 {
        return (modulus, None);
    }
    let (mut a, mut b) = (n as isize, modulus as isize);
    let (mut x00, mut x01) = (1, 0);
    while b != 0 {
        let (q, r) = (a / b, a % b);
        (x00, x01) = (x01, x00 - q * x01);
        (a, b) = (b, r);
    }
    assert!(a > 0);
    let gcd = a as usize;
    if x00 < 0 {
        x00 += (modulus / gcd) as isize;
    }
    let x00 = x00 as usize;
    assert!(x00 < modulus / gcd);
    (gcd, Some(x00))
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
        assert_eq!(super::extended_euclidean_recurse(0, 0), (0, 1, 0));
        assert_eq!(super::extended_euclidean(-30, 111), (3, 11, 3));
        assert_eq!(super::extended_euclidean(0, 0), (0, 1, 0));
        assert_eq!(
            super::extended_euclidean_gcd_modular_inverse(10, 0),
            (10, None)
        );
        assert_eq!(
            super::extended_euclidean_gcd_modular_inverse(5, 2),
            (1, Some(3))
        );
        assert_eq!(
            super::extended_euclidean_gcd_modular_inverse(18, 12),
            (6, Some(2))
        );
        assert_eq!(
            super::extended_euclidean_gcd_modular_inverse(111, 30),
            (3, Some(26)) // 111 / 3 = 37, 30 / 3 = 10, 10^{-1} \equiv 26 \mod 37
        );
    }
}
