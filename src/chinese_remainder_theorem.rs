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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::crt_2_coprime(5, 3, 7, 4), 18);
        assert_eq!(super::crt_2(5, 3, 7, 4), Some(18));
    }
}

// def crt(
//     mod_rem_pairs: list[tuple[int, int]],
// ) -> typing.Optional[int]:
//     mod_rem_pairs = [pair for pair in mod_rem_pairs if pair
// != (1, 0)]     assert len(mod_rem_pairs) >= 1
//     mod, rem = mod_rem_pairs[0]
//     assert 0 <= rem < mod > 1
//     for m, r in mod_rem_pairs[1:]:
//         assert 0 <= r < m > 1
//         result = crt_2(mod, rem, m, r)
//         if result is None:
//             return None
//         rem = result
//         mod = dsalgo.euclidean.least_common_multiple(mod, m)
//         assert 0 <= rem < mod
//     return rem

// def safe_crt_2(
//     mod_0: int,
//     rem_0: int,
//     mod_1: int,
//     rem_1: int,
// ) -> typing.Optional[int]:
//     assert 0 <= rem_0 < mod_0 > 1 and 0 <= rem_1 < mod_1 > 1
//     gcd, inv_u0 =
// dsalgo.euclidean.extended_euclidean_gcd_modular_inverse(
//         mod_1,
//         mod_0 % mod_1,
//     )
//     assert inv_u0 is not None
//     if (rem_1 - rem_0) % gcd:
//         return None
//     u1 = mod_1 // gcd
//     x = ((rem_1 - rem_0) // gcd) % u1 * inv_u0 % u1
//     value = rem_0 + x * mod_0
//     assert max(rem_0, rem_1) <= value < mod_0 * u1
//     return value

// def safe_crt(
//     mod_rem_pairs: list[tuple[int, int]],
// ) -> typing.Optional[int]:
//     mod_rem_pairs = [pair for pair in mod_rem_pairs if pair
// != (1, 0)]     assert len(mod_rem_pairs) >= 1
//     mod, rem = mod_rem_pairs[0]
//     assert 0 <= rem < mod > 1
//     for m, r in mod_rem_pairs[1:]:
//         assert 0 <= r < m > 1
//         result = safe_crt_2(mod, rem, m, r)
//         if result is None:
//             return None
//         rem = result
//         mod = dsalgo.euclidean.least_common_multiple(mod, m)
//         assert 0 <= rem < mod
//     return rem
