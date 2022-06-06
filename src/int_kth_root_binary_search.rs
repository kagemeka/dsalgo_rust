pub fn int_kth_root_binary_search(n: u64, k: u8) -> u64 {
    assert!(k > 0);
    if k == 1 || n <= 1 {
        return n;
    }
    let mut lo = 0;
    let mut hi = n;
    while hi - lo > 1 {
        let x = (lo + hi) >> 1;
        if let Some(y) = x.checked_pow(k as u32) {
            if y <= n {
                lo = x;
            } else {
                hi = x;
            }
        } else {
            hi = x;
        }
    }
    lo
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let pairs = [
            (1, 100),
            (2, 10),
            (3, 4),
            (4, 3),
            (5, 2),
            (6, 2),
            (7, 1),
        ];
        for &(k, ans) in &pairs {
            assert_eq!(
                int_kth_root_binary_search(100, k),
                ans
            );
        }
    }
}
