use crate::miller_rabin_fixed_bases::MillerRabinFixedBases;

pub fn miller_rabin_test(n: u64) -> bool {
    let tester = MillerRabinFixedBases::default();
    tester.is_prime(n)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert!(miller_rabin_test(998_244_353));
        assert!(miller_rabin_test(
            1_000_000_007
        ));
        assert!(miller_rabin_test(
            1_000_000_007
        ));
        assert!(!miller_rabin_test(561));
        assert!(!miller_rabin_test(512_461));
    }
}
