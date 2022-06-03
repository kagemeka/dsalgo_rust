use crate::{
    miller_rabin_fixed_bases::miller_rabin_fixed_bases,
    miller_rabin_precise_bases::BASES_64_FEW,
};

pub fn miller_rabin_test(n: u64) -> bool {
    miller_rabin_fixed_bases(&BASES_64_FEW, n)
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
