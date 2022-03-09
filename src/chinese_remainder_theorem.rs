use crate::euclidean;

pub fn crt_2_coprime(mod_0: usize, rem_0: usize, mod_1: usize, rem_1: usize) -> usize {
    assert!(mod_0 > 1 && rem_0 < mod_0 && mod_1 > 1 && rem_1 < mod_1);
    crt_2(mod_0, rem_0, mod_1, rem_1).unwrap()
}

pub fn crt_2(
    mut mod_0: usize,
    mut rem_0: usize,
    mut mod_1: usize,
    mut rem_1: usize,
) -> Option<usize> {
    assert!(mod_0 > 1 && rem_0 < mod_0 && mod_1 > 1 && rem_1 < mod_1);
    if rem_0 > rem_1 {
        (rem_0, rem_1) = (rem_1, rem_0);
        (mod_0, mod_1) = (mod_1, mod_0);
    }
    let (gcd, x, _) = euclidean::extended_euclidean_recurse(mod_0 as isize, mod_1 as isize);
    if (rem_1 - rem_0) % gcd != 0 {
        return None;
    }
    let lcm = mod_0 / gcd * mod_1;
    let s = (rem_1 - rem_0) / gcd;
    let x = if x >= 0 { x } else { lcm as isize + x } as usize;
    Some((rem_0 + x * s % lcm * mod_0) % lcm)
}

pub fn safe_crt_2(
    mut mod_0: usize,
    mut rem_0: usize,
    mut mod_1: usize,
    mut rem_1: usize,
) -> Option<usize> {
    assert!(mod_0 > 1 && rem_0 < mod_0 && mod_1 > 1 && rem_1 < mod_1);
    if rem_0 > rem_1 {
        (rem_0, rem_1) = (rem_1, rem_0);
        (mod_0, mod_1) = (mod_1, mod_0);
    }
    let (gcd, inv_u0) = euclidean::extended_euclidean_gcd_modular_inverse(mod_1, mod_0 % mod_1);
    let inv_u0 = inv_u0.unwrap();
    if (rem_1 - rem_0) % gcd != 0 {
        return None;
    }
    let u1 = mod_1 / gcd;
    let x = (rem_1 - rem_0) / gcd * inv_u0 % u1;
    let value = rem_0 + x * mod_0;
    assert!(rem_1 <= value && value < mod_0 * u1);
    Some(value)
}

pub fn safe_crt(mod_rem_pairs: &[(usize, usize)]) -> Option<usize> {
    if mod_rem_pairs.len() == 0 {
        return None;
    }
    let mod_rem_pairs = mod_rem_pairs
        .iter()
        .filter(|&&pair| pair != (1, 0))
        .collect::<Vec<_>>();
    if mod_rem_pairs.len() == 0 {
        return Some(0);
    }
    let (mut modulus, mut remainder) = mod_rem_pairs[0];
    assert!(modulus > 1 && remainder < modulus);
    for &&(m, r) in mod_rem_pairs.iter().skip(1) {
        assert!(m > 1 && r < m);
        if let Some(result) = safe_crt_2(modulus, remainder, m, r) {
            remainder = result;
            modulus = euclidean::least_common_multiple(modulus, m);
            assert!(remainder < modulus);
        } else {
            return None;
        }
    }
    return Some(remainder);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::crt_2_coprime(5, 3, 7, 4), 18);
        assert_eq!(super::crt_2(5, 3, 7, 4), Some(18));
        assert_eq!(super::safe_crt_2(5, 3, 7, 4), Some(18));
        let mod_rem_pairs = vec![(5, 3), (7, 4), (8, 3)];
        assert_eq!(super::safe_crt(&mod_rem_pairs), Some(123));
    }
}
