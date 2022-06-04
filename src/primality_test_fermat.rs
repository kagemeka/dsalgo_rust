use crate::fermat_test_fixed_bases::FermatTestFixedBases;

pub fn fermat_test(n: u64, epochs: u8) -> bool {
    let tester = FermatTestFixedBases::from_random_bases(epochs);
    tester.is_prime(n)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(
            super::fermat_test(998_244_353, 10),
            true
        );
        assert_eq!(
            super::fermat_test(1_000_000_007, 10),
            true
        );
        assert_eq!(
            super::fermat_test(561, 10),
            false
        );
        assert_eq!(
            super::fermat_test(512461, 10),
            false
        );
    }
}
