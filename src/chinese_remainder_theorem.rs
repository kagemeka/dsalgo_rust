use crate::{
    extended_euclidean_algorithm::extgcd,
    extended_euclidean_modular_gcd_inverse::euclidean_mod_gcd_inv,
};

// fn crt_pre_swap(
//     mod_0: &mut u64,
//     rem_0: &mut u64,
//     mod_1: &mut u64,
//     rem_1: &mut u64,
// ) {
//     assert!(rem_0 < mod_0 && rem_1 < mod_1);
//     if rem_0 < rem_1 {
//         std::mem::swap(rem_0, rem_1);
//         std::mem::swap(mod_0, mod_1);
//     }
// }

pub fn crt(
    mut mod_0: u64,
    mut rem_0: u64,
    mut mod_1: u64,
    mut rem_1: u64,
) -> Result<(u64, u64), String> {
    assert!(rem_0 < mod_0 && rem_1 < mod_1);
    let (gcd, mut x, _) = extgcd(mod_0 as i64, mod_1 as i64);
    if rem_0 > rem_1 {
        std::mem::swap(&mut rem_0, &mut rem_1);
        std::mem::swap(&mut mod_0, &mut mod_1);
    }
    if (rem_1 - rem_0) % gcd != 0 {
        return Err("answer is undefined".to_string());
    }
    let lcm = mod_0 / gcd * mod_1;
    let s = (rem_1 - rem_0) / gcd;
    if x < 0 {
        x += lcm as i64;
    }
    Ok((
        lcm,
        (x as u64 * s % lcm * mod_0 + rem_0) % lcm,
    ))
}

pub fn crt_coprime(
    mod_0: u64,
    rem_0: u64,
    mod_1: u64,
    rem_1: u64,
) -> (u64, u64) {
    safe_crt(mod_0, rem_0, mod_1, rem_1).unwrap()
}

/// avoid overflows unless lcm(mod_0, mod_1) overflows.
pub fn safe_crt(
    mut mod_0: u64,
    mut rem_0: u64,
    mut mod_1: u64,
    mut rem_1: u64,
) -> Result<(u64, u64), String> {
    assert!(rem_0 < mod_0 && rem_1 < mod_1);
    if rem_0 > rem_1 {
        std::mem::swap(&mut rem_0, &mut rem_1);
        std::mem::swap(&mut mod_0, &mut mod_1);
    }
    if mod_0 % mod_1 == 0 {
        // extgcd_modinv(*, 0) fails to panic.
        // because it's trivial that inv(0) is undefined.
        // (mod_1|mod_0 -> rem_1 <= rem_0) \land rem_0 <= rem_1.
        return if rem_0 == rem_1 {
            Ok((mod_0, rem_0))
        } else {
            Err("answer is undefined".to_string())
        };
    }

    let (gcd, inv_u0) = euclidean_mod_gcd_inv(
        mod_1 as u64,
        (mod_0 % mod_1) as u64,
    );
    if (rem_1 - rem_0) % gcd != 0 {
        return Err("answer is undefined".to_string());
    }
    let u1 = mod_1 / gcd;
    let x = (rem_1 - rem_0) / gcd * inv_u0 % u1;
    let ans = rem_0 + x * mod_0;
    let lcm = mod_0 * u1;
    debug_assert!(rem_1 <= ans && ans < lcm);
    Ok((lcm, ans))
}

pub fn safe_crt_reduce(
    mod_rem_pairs: &[(u64, u64)],
) -> Result<(u64, u64), String> {
    let mut iter = mod_rem_pairs.iter();
    let (mut mod_0, mut rem_0) = iter.next().unwrap();
    for &(mod_1, rem_1) in iter {
        let (lcm, ans) = safe_crt(mod_0, rem_0, mod_1, rem_1)?;
        mod_0 = lcm;
        rem_0 = ans;
        debug_assert!(rem_0 < mod_0);
    }
    Ok((mod_0, rem_0))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(crt(5, 3, 7, 4), Ok((35, 18)),);
        assert_eq!(
            crt_coprime(5, 3, 7, 4),
            (35, 18),
        );
        assert_eq!(
            safe_crt(5, 3, 7, 4),
            Ok((35, 18)),
        );
        let mod_rem_pairs = vec![(5, 3), (7, 4), (8, 3)];
        assert_eq!(
            safe_crt_reduce(&mod_rem_pairs),
            Ok((280, 123)),
        );
    }
}
