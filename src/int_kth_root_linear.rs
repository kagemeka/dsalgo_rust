/// for k >= 10, faster than binary search.
pub fn int_kth_root_linear(n: u64, k: u8) -> u64 {
    assert!(k > 0);
    let mut x: u64 = 0;
    while let Some(y) = x.checked_pow(k as u32) {
        if y > n {
            break;
        }
        x += 1;
    }
    x - 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let x = std::u64::MAX;
        let pairs = [
            (10, 84),
            (11, 56),
            (12, 40),
            (13, 30),
            (14, 23),
            (15, 19),
            (16, 15),
            (17, 13),
            (18, 11),
            (19, 10),
            (20, 9),
        ];

        for &(k, ans) in &pairs {
            assert_eq!(int_kth_root_linear(x, k), ans);
        }
    }
}
