pub const MILLER_RABIN_BASES_32: [u64; 3] = [2, 7, 61];
pub const MILLER_RABIN_BASES_64: [u64; 12] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37,
];
pub const MILLER_RABIN_BASES_64_FEW: [u64; 7] = [
    2,
    325,
    9_375,
    28_178,
    450_775,
    9_780_504,
    1_795_265_022,
];

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
    }
}
